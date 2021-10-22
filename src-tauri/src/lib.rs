// Copyright 2019-2021 Manta Network.
// This file is part of manta-signer.
//
// manta-signer is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// manta-signer is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with manta-signer. If not, see <http://www.gnu.org/licenses/>.

//! Manta Signer Server

#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![forbid(rustdoc::broken_intra_doc_links)]
#![forbid(missing_docs)]

use ark_serialize::CanonicalDeserialize;
use async_std::{
    fs::{self, File},
    io::{self, ReadExt, WriteExt},
};
use bip0039::{Count, Mnemonic};
use bip32::XPrv;
use cocoon::{Cocoon, Error as CocoonError};
use codec::{Decode, Encode};
use core::convert::TryInto;
use manta_api::{
    DeriveShieldedAddressParams, GenerateAssetParams, GeneratePrivateTransferBatchParams,
    GeneratePrivateTransferParams, GenerateReclaimBatchParams, GenerateReclaimParams,
    MantaRootSeed, PrivateTransferBatch, ReclaimBatch, RecoverAccountParams,
};
use manta_asset::{AssetId, MantaAsset, MantaAssetShieldedAddress, Process, Sampling};
use manta_crypto::{
    commitment_parameters, leaf_parameters, two_to_one_parameters, Groth16Pk, MantaSerDes,
};
use manta_data::{BuildMetadata, PrivateTransferData, ReclaimData};
use rand::{thread_rng, CryptoRng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{
    async_runtime::{channel, Mutex, Receiver, RwLock, Sender},
    Window,
};
use tide::{
    listener::ToListener, Body, Error, Request as ServerRequest, Result as ServerResult, Server,
    StatusCode,
};

/// Ensure that `$expr` is `Ok(_)` and if not returns a [`StatusCode::InternalServerError`].
macro_rules! ensure {
    ($expr:expr) => {
        ensure!($expr, "")
    };
    ($expr:expr, $msg:expr) => {
        core::result::Result::map_err($expr, move |_| {
            Error::from_str(StatusCode::InternalServerError, $msg)
        })
    };
}

/// Password
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Password {
    /// Known Password
    Known(String),

    /// Unknown Password
    Unknown,
}

impl Password {
    /// Returns the inner password, if it is known.
    #[inline]
    pub fn known(self) -> Option<String> {
        match self {
            Self::Known(password) => Some(password),
            _ => None,
        }
    }
}

impl Default for Password {
    #[inline]
    fn default() -> Self {
        Self::Unknown
    }
}

/// Password Storage Type
type PasswordStoreType = Arc<RwLock<Option<Sender<Password>>>>;

/// Password Storage Handle
pub struct PasswordStoreHandle(PasswordStoreType);

impl PasswordStoreHandle {
    /// Builds a new password storage system, waiting on the receiver to receive it's first
    /// message.
    #[inline]
    pub async fn setup(self) -> Receiver<Password> {
        let (sender, mut receiver) = channel(1);
        *self.0.write().await = Some(sender);
        receiver.recv().await;
        receiver
    }
}

/// Password Storage
#[derive(Default)]
pub struct PasswordStore(PasswordStoreType);

impl PasswordStore {
    /// Returns a handle for setting up a [`PasswordStore`].
    #[inline]
    pub fn handle(&self) -> PasswordStoreHandle {
        PasswordStoreHandle(self.0.clone())
    }

    /// Loads a new password into the state.
    #[inline]
    pub async fn load(&self, password: String) {
        if let Some(state) = &*self.0.read().await {
            state.send(Password::Known(password)).await.unwrap();
        }
    }

    /// Clears the password state.
    #[inline]
    pub async fn clear(&self) {
        if let Some(state) = &*self.0.read().await {
            state.send(Password::Unknown).await.unwrap();
        }
    }
}
/// Root Seed
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RootSeed(pub MantaRootSeed);

impl RootSeed {
    /// Builds a new [`RootSeed`] from a `mnemonic`.
    #[inline]
    pub fn new(mnemonic: &Mnemonic) -> Self {
        Self(mnemonic.to_seed(""))
    }

    /// Saves `self` to the standard root seed file, encrypting it with `password`.
    #[inline]
    pub async fn save(self, password: String) -> Result<(), CocoonError> {
        let mut data = Vec::new();
        Cocoon::new(password.as_bytes()).dump(self.0.to_vec(), &mut data)?;
        File::create("root_seed.aes")
            .await
            .map_err(CocoonError::Io)?
            .write_all(&data)
            .await
            .map_err(CocoonError::Io)
    }

    /// Loads `self` from the standard root seed file, decrypting it with `password`.
    #[inline]
    pub async fn load(password: String) -> Result<Self, CocoonError> {
        let mut data = Vec::new();
        File::open("root_seed.aes")
            .await
            .map_err(CocoonError::Io)?
            .read_to_end(&mut data);
        Ok(Self(
            Cocoon::new(password.as_bytes())
                .parse(&mut data.as_slice())?
                .try_into()
                .expect(""),
        ))
    }
}

impl Default for RootSeed {
    #[inline]
    fn default() -> Self {
        Self([Default::default(); 64])
    }
}

/// Checks if a root seed exists at the canonical file path.
#[inline]
pub async fn account_exists() -> io::Result<bool> {
    Ok(fs::metadata("root_seed.aes").await?.is_file())
}

/// Creates a new account by building and saving a new root seed from the given `password`.
#[inline]
pub async fn create_account(password: String) -> Result<Mnemonic, CocoonError> {
    let mnemonic = Mnemonic::generate(Count::Words12);
    RootSeed::new(&mnemonic).save(password).await?;
    Ok(mnemonic)
}

/// Transaction Type
#[derive(Deserialize, Serialize)]
pub enum TransactionType {
    /// Private Transfer
    PrivateTransfer {
        /// Recipient Address
        recipient: String,
    },

    /// Reclaim
    Reclaim,
}

/// Transaction Summary
#[derive(Deserialize, Serialize)]
pub struct TransactionSummary {
    /// Transaction Type
    pub transaction: TransactionType,

    /// Transaction Amount
    pub amount: String,

    /// Currency Symbol
    pub currency_symbol: String,
}

/// Inner State
struct InnerState {
    /// Main Window
    window: Window,

    /// Password Receiver
    password: Receiver<Password>,

    /// Current Root Seed
    root_seed: Option<RootSeed>,
}

impl InnerState {
    /// Builds a new [`InnerState`] from `window` and `password`.
    #[inline]
    fn new(window: Window, password: Receiver<Password>) -> Self {
        Self {
            window,
            password,
            root_seed: None,
        }
    }

    /// Returns the password from the user, prompted with `prompt`.
    #[inline]
    async fn authorize<T>(&mut self, prompt: T) -> Option<Password>
    where
        T: Serialize,
    {
        self.window.emit("authorize", prompt).unwrap();
        self.window.show().unwrap();
        self.password.recv().await
    }

    /// Sets the inner seed from the output of a call to [`Self::authorize`] using the given
    /// `prompt`.
    #[inline]
    async fn set_seed_from_authorization<T>(&mut self, prompt: T) -> Option<()>
    where
        T: Serialize,
    {
        if let Password::Known(password) = self.authorize(prompt).await? {
            self.root_seed = RootSeed::load(password).await.ok();
        }
        Some(())
    }

    /// Returns the stored root seed if it exists, otherwise, gets the password from the user
    /// and tries to decrypt the root seed.
    #[inline]
    async fn get_root_seed<T>(&mut self, prompt: T) -> Option<RootSeed>
    where
        T: Serialize,
    {
        if self.root_seed.is_none() {
            self.set_seed_from_authorization(prompt).await?;
        }
        self.root_seed
    }

    /// Returns the currently stored root seed if it matches the one returned by the user after
    /// prompting.
    #[inline]
    async fn check_root_seed<T>(&mut self, prompt: T) -> Option<RootSeed>
    where
        T: Serialize,
    {
        match self.root_seed {
            Some(current_root_seed) => {
                let password = self.authorize(prompt).await?.known()?;
                if current_root_seed == RootSeed::load(password).await.ok()? {
                    Some(current_root_seed)
                } else {
                    None
                }
            }
            _ => {
                self.set_seed_from_authorization(prompt).await?;
                self.root_seed
            }
        }
    }
}

/// Signer State
#[derive(Clone)]
pub struct State(Arc<Mutex<InnerState>>);

impl State {
    /// Builds a new [`State`] using `window` and `password`.
    #[inline]
    pub fn new(window: Window, password: Receiver<Password>) -> Self {
        Self(Arc::new(Mutex::new(InnerState::new(window, password))))
    }

    /// Returns the stored root seed if it exists, otherwise, gets the password from the user
    /// and tries to decrypt the root seed.
    #[inline]
    async fn get_root_seed<T>(&self, prompt: T) -> Option<RootSeed>
    where
        T: Serialize,
    {
        self.0.lock().await.get_root_seed(prompt).await
    }

    /// Returns the currently stored root seed if it matches the one returned by the user after
    /// prompting.
    #[inline]
    async fn check_root_seed<T>(&self, prompt: T) -> Option<RootSeed>
    where
        T: Serialize,
    {
        self.0.lock().await.check_root_seed(prompt).await
    }
}

/// Server Request Type
pub type Request = ServerRequest<State>;

/// Signer Service
pub struct Service(Server<State>);

impl Service {
    /// Builds a new [`Service`] from `window` and `password`.
    #[inline]
    pub fn build(window: Window, password: Receiver<Password>) -> Self {
        let mut server = Server::with_state(State::new(window, password));
        server.at("/heartbeat").get(Self::heartbeat);
        server.at("/recoverAccount").post(Self::recover_account);
        server
            .at("/deriveShieldedAddress")
            .post(Self::derive_shielded_address);
        server.at("/generateAsset").post(Self::generate_asset);
        server.at("/generateMintData").post(Self::mint);
        server
            .at("/generatePrivateTransferData")
            .post(Self::private_transfer);
        server.at("/generateReclaimData").post(Self::reclaim);
        Self(server)
    }

    /// Starts the service on `listener`.
    #[inline]
    pub async fn serve<L>(self, listener: L) -> io::Result<()>
    where
        L: ToListener<State>,
    {
        self.0.listen(listener).await
    }

    /// Sends a heartbeat to the client.
    #[inline]
    async fn heartbeat(request: Request) -> ServerResult<String> {
        let _ = request;
        Ok(String::from("heartbeat"))
    }

    /// Runs an account recovery for the given `request`.
    #[inline]
    async fn recover_account(mut request: Request) -> ServerResult {
        let (body, state) = Self::process(&mut request).await?;
        let params = ensure!(RecoverAccountParams::decode(&mut body.as_slice()))?;
        let root_seed = ensure!(state.get_root_seed("recover_account").await.ok_or(()))?;
        let recovered_account = manta_api::recover_account(params, &root_seed.0).encode();
        Ok(Body::from_json(&RecoverAccountMessage::new(recovered_account))?.into())
    }

    /// Generates a new derived shielded address for the given `request`.
    #[inline]
    async fn derive_shielded_address(mut request: Request) -> ServerResult {
        let (body, state) = Self::process(&mut request).await?;
        let params = ensure!(DeriveShieldedAddressParams::decode(&mut body.as_slice(),))?;
        let root_seed = ensure!(state
            .get_root_seed("derive_shielded_address")
            .await
            .ok_or(()))?;
        let mut address = Vec::new();
        ensure!(manta_api::derive_shielded_address(params, &root_seed.0).serialize(&mut address))?;
        Ok(Body::from_json(&ShieldedAddressMessage::new(address))?.into())
    }

    /// Generates an asset for the given `request`.
    #[inline]
    async fn generate_asset(mut request: Request) -> ServerResult {
        let (body, state) = Self::process(&mut request).await?;
        let params = ensure!(GenerateAssetParams::decode(&mut body.as_slice()))?;
        let root_seed = ensure!(state.get_root_seed("generate_asset").await.ok_or(()))?;
        let asset = manta_api::generate_signer_input_asset(params, &root_seed.0).encode();
        Ok(Body::from_json(&AssetMessage::new(asset))?.into())
    }

    /// Generates mint data for the given `request`.
    #[inline]
    async fn mint(mut request: Request) -> ServerResult {
        let (body, state) = Self::process(&mut request).await?;
        let params = ensure!(GenerateAssetParams::decode(&mut body.as_slice()))?;
        let root_seed = ensure!(state.get_root_seed("mint").await.ok_or(()))?;
        let mut mint_data = Vec::new();
        ensure!(manta_api::generate_mint_data(params, &root_seed.0).serialize(&mut mint_data))?;
        Ok(Body::from_json(&MintMessage::new(mint_data))?.into())
    }

    /// Generates private transfer data for the given `request`.
    #[inline]
    async fn private_transfer(mut request: Request) -> ServerResult {
        let (body, state) = Self::process(&mut request).await?;
        let params = ensure!(GeneratePrivateTransferBatchParams::decode(
            &mut body.as_slice()
        ))?;
        let root_seed = ensure!(state.check_root_seed("private_transfer").await.ok_or(()))?;
        let mut rng = ChaCha20Rng::from_rng(thread_rng()).expect("Unable to sample RNG.");
        let private_transfer_data =
            batch_generate_private_transfer_data(params, &root_seed, "transfer_pk.bin", &mut rng)
                .await
                .encode();
        Ok(Body::from_json(&PrivateTransferMessage::new(private_transfer_data))?.into())
    }

    /// Generates reclaim data for the given `request`.
    #[inline]
    async fn reclaim(mut request: Request) -> ServerResult {
        let (body, state) = Self::process(&mut request).await?;
        let params = ensure!(GenerateReclaimBatchParams::decode(&mut body.as_slice()))?;
        let root_seed = ensure!(state.check_root_seed("private_transfer").await.ok_or(()))?;
        let mut rng = ChaCha20Rng::from_rng(thread_rng()).expect("Unable to sample RNG.");
        let reclaim_data = batch_generate_reclaim_data(
            params,
            &root_seed,
            "transfer_pk.bin",
            "reclaim_pk.bin",
            &mut rng,
        )
        .await
        .encode();
        Ok(Body::from_json(&ReclaimMessage::new(reclaim_data))?.into())
    }

    /// Preprocesses a `request`, extracting the body as a byte vector and returning the
    /// internal state.
    #[inline]
    async fn process(request: &mut Request) -> ServerResult<(Vec<u8>, &State)> {
        Ok((request.body_bytes().await?, request.state()))
    }
}

/// Shielded Address Message
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShieldedAddressMessage {
    /// Address
    pub address: Vec<u8>,

    /// Version
    pub version: String,
}

impl ShieldedAddressMessage {
    /// Builds a new [`ShieldedAddressMessage`].
    #[inline]
    pub fn new(address: Vec<u8>) -> Self {
        Self {
            address,
            version: "0.0.0".into(),
        }
    }
}

/// Recover Account Message
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RecoverAccountMessage {
    /// Recovered Account
    pub recovered_account: Vec<u8>,

    /// Version
    pub version: String,
}

impl RecoverAccountMessage {
    /// Builds a new [`RecoverAccountMessage`].
    #[inline]
    pub fn new(recovered_account: Vec<u8>) -> Self {
        Self {
            recovered_account,
            version: "0.0.0".into(),
        }
    }
}

/// Asset Message
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetMessage {
    /// Asset
    pub asset: Vec<u8>,

    /// Version
    pub version: String,
}

impl AssetMessage {
    /// Builds a new [`AssetMessage`].
    #[inline]
    pub fn new(asset: Vec<u8>) -> Self {
        Self {
            asset,
            version: "0.0.0".into(),
        }
    }
}

/// Mint Message
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MintMessage {
    /// Mint Data
    pub mint_data: Vec<u8>,

    /// Version
    pub version: String,
}

impl MintMessage {
    /// Builds a new [`MintMessage`].
    #[inline]
    pub fn new(mint_data: Vec<u8>) -> Self {
        Self {
            mint_data,
            version: "0.0.0".into(),
        }
    }
}

/// Private Transfer Message
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateTransferMessage {
    /// Private Transfer Data
    pub private_transfer_data: Vec<u8>,

    /// Version
    pub version: String,
}

impl PrivateTransferMessage {
    /// Builds a new [`PrivateTransferMessage`].
    #[inline]
    pub fn new(private_transfer_data: Vec<u8>) -> Self {
        Self {
            private_transfer_data,
            version: "0.0.0".into(),
        }
    }
}

/// Reclaim Message
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReclaimMessage {
    /// Reclaim Data
    pub reclaim_data: Vec<u8>,

    /// Version
    pub version: String,
}

impl ReclaimMessage {
    /// Builds a new [`ReclaimMessage`].
    #[inline]
    pub fn new(reclaim_data: Vec<u8>) -> Self {
        Self {
            reclaim_data,
            version: "0.0.0".into(),
        }
    }
}

/// Loads proving key from `path`.
#[inline]
pub async fn load_proving_key(path: &str) -> Groth16Pk {
    Groth16Pk::deserialize_unchecked(
        fs::read(path)
            .await
            .expect("Failed to read file.")
            .as_slice(),
    )
    .expect("Failed to deserialize proving key.")
}

/// Generates batched private transfer data.
#[inline]
pub async fn batch_generate_private_transfer_data<R>(
    params: GeneratePrivateTransferBatchParams,
    root_seed: &RootSeed,
    private_transfer_pk_path: &str,
    rng: &mut R,
) -> PrivateTransferBatch
where
    R: CryptoRng + RngCore,
{
    let private_transfer_pk = load_proving_key(private_transfer_pk_path).await;
    let asset_id = params.asset_id;
    let receiving_address = params.receiving_address;
    let last_private_transfer_index = params.private_transfer_params_list.len() - 1;
    PrivateTransferBatch {
        private_transfer_data_list: params
            .private_transfer_params_list
            .into_iter()
            .enumerate()
            .map(|(i, p)| {
                let receiving_address = if i == last_private_transfer_index {
                    Some(receiving_address)
                } else {
                    None
                };
                generate_private_transfer_data(
                    p,
                    asset_id,
                    receiving_address,
                    root_seed,
                    &private_transfer_pk,
                    rng,
                )
            })
            .collect(),
    }
}

/// Generates batched reclaim data.
#[inline]
pub async fn batch_generate_reclaim_data<R>(
    params: GenerateReclaimBatchParams,
    root_seed: &RootSeed,
    private_transfer_pk_path: &str,
    reclaim_pk_path: &str,
    rng: &mut R,
) -> ReclaimBatch
where
    R: CryptoRng + RngCore,
{
    let private_transfer_pk = load_proving_key(private_transfer_pk_path).await;
    let reclaim_pk = load_proving_key(reclaim_pk_path).await;
    let asset_id = params.asset_id;
    ReclaimBatch {
        reclaim_data: generate_reclaim_data(
            params.reclaim_params,
            asset_id,
            &root_seed.0,
            &reclaim_pk,
            rng,
        ),
        private_transfer_data_list: params
            .private_transfer_params_list
            .into_iter()
            .map(|p| {
                generate_private_transfer_data(
                    p,
                    asset_id,
                    None,
                    root_seed,
                    &private_transfer_pk,
                    rng,
                )
            })
            .collect(),
    }
}

/// Generates one round of private transfer data.
#[inline]
pub fn generate_private_transfer_data<R>(
    params: GeneratePrivateTransferParams,
    asset_id: AssetId,
    receiving_address: Option<MantaAssetShieldedAddress>,
    root_seed: &RootSeed,
    transfer_pk: &Groth16Pk,
    rng: &mut R,
) -> PrivateTransferData
where
    R: CryptoRng + RngCore,
{
    let commit_params = commitment_parameters();
    let leaf_params = leaf_parameters();
    let two_to_one_params = two_to_one_parameters();
    let sender_asset_1 = generate_asset(
        GenerateAssetParams {
            asset_id,
            value: params.sender_asset_1_value,
            keypath: params.sender_asset_1_keypath,
        },
        &root_seed.0,
    );
    let sender_asset_2 = generate_asset(
        GenerateAssetParams {
            asset_id,
            value: params.sender_asset_2_value,
            keypath: params.sender_asset_2_keypath,
        },
        &root_seed.0,
    );
    let sender_metadata_1 = sender_asset_1
        .build(
            &leaf_params,
            &two_to_one_params,
            &params.sender_asset_1_shard,
        )
        .expect("Failed to build sender 1 metadata");
    let sender_metadata_2 = sender_asset_2
        .build(
            &leaf_params,
            &two_to_one_params,
            &params.sender_asset_2_shard,
        )
        .expect("Failed to build sender 2 metadata");
    let non_change_receiving_address = match receiving_address {
        Some(receiving_address) => receiving_address,
        None => manta_api::derive_shielded_address(
            DeriveShieldedAddressParams {
                keypath: params.non_change_output_keypath.unwrap(),
                asset_id,
            },
            &root_seed.0,
        ),
    };
    let non_change_processed_receiver = non_change_receiving_address
        .process(&params.non_change_output_value, rng)
        .expect("Failed to process receiver 1 shielded address");
    let change_address = manta_api::derive_shielded_address(
        DeriveShieldedAddressParams {
            keypath: params.change_output_keypath,
            asset_id,
        },
        &root_seed.0,
    );
    let change_processed_receiver = change_address
        .process(&params.change_output_value, rng)
        .expect("Failed to process receiver 2 shielded address");
    manta_api::generate_private_transfer_struct(
        commit_params,
        leaf_params,
        two_to_one_params,
        transfer_pk,
        [sender_metadata_1, sender_metadata_2],
        [non_change_processed_receiver, change_processed_receiver],
        rng,
    )
    .expect("Failed to generate private transfer payload from deserialized data")
}

/// Generates one round of reclaim data.
#[inline]
pub fn generate_reclaim_data<R>(
    params: GenerateReclaimParams,
    asset_id: AssetId,
    root_seed: &MantaRootSeed,
    reclaim_pk: &Groth16Pk,
    rng: &mut R,
) -> ReclaimData
where
    R: CryptoRng + RngCore,
{
    let commit_params = commitment_parameters();
    let leaf_params = leaf_parameters();
    let two_to_one_params = two_to_one_parameters();
    let input_asset_1 = generate_asset(
        GenerateAssetParams {
            asset_id,
            value: params.input_asset_1_value,
            keypath: params.input_asset_1_keypath,
        },
        root_seed,
    );
    let input_asset_2 = generate_asset(
        GenerateAssetParams {
            asset_id,
            value: params.input_asset_2_value,
            keypath: params.input_asset_2_keypath,
        },
        root_seed,
    );
    let sender_metadata_1 = input_asset_1
        .build(
            &leaf_params,
            &two_to_one_params,
            &params.input_asset_1_shard,
        )
        .expect("Failed to build sender 1 metadata");
    let sender_metadata_2 = input_asset_2
        .build(
            &leaf_params,
            &two_to_one_params,
            &params.input_asset_2_shard,
        )
        .expect("Failed to build sender 2 metadata");

    let change_address = manta_api::derive_shielded_address(
        DeriveShieldedAddressParams {
            keypath: params.change_keypath,
            asset_id,
        },
        root_seed,
    );
    let change_value =
        params.input_asset_1_value + params.input_asset_2_value - params.reclaim_value;
    let change_processed_receiver = change_address
        .process(&change_value, rng)
        .expect("Failed to build processed receiver");
    manta_api::generate_reclaim_struct(
        commit_params,
        leaf_params,
        two_to_one_params,
        reclaim_pk,
        [sender_metadata_1, sender_metadata_2],
        change_processed_receiver,
        params.reclaim_value,
        rng,
    )
    .expect("Failed to generate reclaim struct")
}

///
#[inline]
pub fn generate_asset(params: GenerateAssetParams, root_seed: &MantaRootSeed) -> MantaAsset {
    let asset_secret_key = XPrv::derive_from_path(
        root_seed.as_ref(),
        &params.keypath.parse().expect("Invalid derivation path"),
    )
    .expect("Failed to derive extended private key from path")
    .private_key()
    .to_bytes()
    .try_into()
    .unwrap();
    MantaAsset::sample(
        &commitment_parameters(),
        &asset_secret_key,
        &params.asset_id,
        &params.value,
    )
    .expect("Failed to sample asset")
}
