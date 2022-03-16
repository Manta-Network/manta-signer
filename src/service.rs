// Copyright 2019-2022 Manta Network.
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

//! Manta Signer Service

// TODO: Add better logging.

use crate::{
    batching::{batch_generate_private_transfer_data, batch_generate_reclaim_data},
    config::Config,
    secret::{Authorizer, Password, RootSeed, SecretString},
};
use async_std::{
    io::{self, WriteExt},
    sync::Mutex,
    task::sleep,
};
use codec::{Decode, Encode};
use core::time::Duration;
use http_types::headers::HeaderValue;
use futures::StreamExt;
use manta_api::{
    get_private_transfer_batch_params_currency_symbol, get_private_transfer_batch_params_recipient,
    get_private_transfer_batch_params_value, get_reclaim_batch_params_currency_symbol,
    get_reclaim_batch_params_value, DeriveShieldedAddressParams, GenerateAssetParams,
    GeneratePrivateTransferBatchParams, GenerateReclaimBatchParams, RecoverAccountParams,
};
use manta_asset::AssetId;
use manta_crypto::MantaSerDes;
use rand::{thread_rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;
use tide::{
    security::{CorsMiddleware, Origin},
    Error, Request as ServerRequest, Result as ServerResult, Server, StatusCode,
};
use tide_websockets::{Message, WebSocket};

/// Manta Signer Server Version
const VERSION: &str = env!("CARGO_PKG_VERSION");

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

/// Returns the currency symbol for the given `asset_id`.
#[inline]
pub fn get_currency_symbol_by_asset_id(asset_id: AssetId) -> Option<&'static str> {
    Some(match asset_id {
        1 => "DOT",
        2 => "KSM",
        _ => return None,
    })
}

/// Sets the task to sleep to delay password retry.
#[inline]
async fn delay_password_retry() {
    sleep(Duration::from_millis(1000)).await;
}

/// Prompt
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields, tag = "type")]
pub enum Prompt {
    /// Recover Account
    RecoverAccount,

    /// Derive Shielded Address
    DeriveShieldedAddress,

    /// Generate Asset
    GenerateAsset,

    /// Mint
    Mint,

    /// Private Transfer
    PrivateTransfer {
        /// Transaction Recipient
        recipient: String,

        /// Transaction Amount
        amount: String,

        /// Currency Symbol
        currency_symbol: Option<&'static str>,
    },

    /// Reclaim
    Reclaim {
        /// Transaction Amount
        amount: String,

        /// Currency Symbol
        currency_symbol: Option<&'static str>,
    },
}

impl From<&GeneratePrivateTransferBatchParams> for Prompt {
    #[inline]
    fn from(params: &GeneratePrivateTransferBatchParams) -> Self {
        Self::PrivateTransfer {
            recipient: get_private_transfer_batch_params_recipient(params),
            amount: get_private_transfer_batch_params_value(params),
            currency_symbol: get_private_transfer_batch_params_currency_symbol(params),
        }
    }
}

impl From<&GenerateReclaimBatchParams> for Prompt {
    #[inline]
    fn from(params: &GenerateReclaimBatchParams) -> Self {
        Self::Reclaim {
            amount: get_reclaim_batch_params_value(params),
            currency_symbol: get_reclaim_batch_params_currency_symbol(params),
        }
    }
}

/// Inner State
///
/// The inner state of the server contains a copy of the server configuration as well as the
/// currently known root seed an access to an [`Authorizer`] future which can reconfirm the root
/// seed.
struct InnerState<A>
where
    A: Authorizer,
{
    /// Server Configuration
    pub config: Config,

    /// Authorizer
    pub authorizer: A,

    /// Current Root Seed
    root_seed: Option<RootSeed>,
}

impl<A> InnerState<A>
where
    A: Authorizer,
{
    /// Builds a new [`InnerState`] from `config` and `authorizer`.
    #[inline]
    fn new(config: Config, authorizer: A) -> Self {
        Self {
            config,
            authorizer,
            root_seed: None,
        }
    }

    /// Loads the root seed from `root_seed_file` with `password`.
    #[inline]
    async fn load_seed(&self, password: &SecretString) -> Option<RootSeed> {
        RootSeed::load(&self.config.root_seed_file, password)
            .await
            .ok()
    }

    /// Sets the inner seed from a given `password`.
    #[inline]
    async fn set_seed(&mut self, password: &SecretString) {
        self.root_seed = self.load_seed(password).await;
    }

    /// Sets the inner seed from a given `password`.
    #[inline]
    async fn set_seed_from_password(&mut self, password: Password) -> Option<RootSeed> {
        if let Some(password) = password.known() {
            self.set_seed(&password).await;
        }
        self.root_seed.clone()
    }

    /// Sets the inner seed from the output of a call to [`Authorizer::password`].
    #[inline]
    async fn set_seed_from_authorization(&mut self) -> Option<RootSeed> {
        let password = self.authorizer.password().await;
        self.set_seed_from_password(password).await
    }

    /// Checks that the starting password can decrypt the root seed file.
    #[inline]
    async fn check_starting_password(&mut self) -> bool {
        self.set_seed_from_authorization().await.is_some()
    }

    /// Returns the stored root seed if it exists, otherwise, gets the password from the user
    /// and tries to decrypt the root seed.
    #[inline]
    async fn get_root_seed(&mut self, prompt: A::Prompt) -> Option<RootSeed> {
        if self.root_seed.is_none() {
            self.authorizer.wake(prompt).await;
            loop {
                if let Some(password) = self.authorizer.password().await.known() {
                    if let Some(root_seed) = self.load_seed(&password).await {
                        self.root_seed = Some(root_seed);
                        self.authorizer.success(Default::default()).await;
                        break;
                    }
                } else {
                    return None;
                }
                delay_password_retry().await;
            }
        }
        self.root_seed.clone()
    }

    /// Returns the currently stored root seed if it matches the one returned by the user after
    /// prompting.
    #[inline]
    async fn check_root_seed(&mut self, prompt: A::Prompt) -> Option<RootSeed> {
        match &self.root_seed {
            Some(current_root_seed) => {
                self.authorizer.wake(prompt).await;
                loop {
                    if let Some(password) = self.authorizer.password().await.known() {
                        if let Some(root_seed) = self.load_seed(&password).await {
                            if current_root_seed == &root_seed {
                                self.authorizer.success(Default::default()).await;
                                return Some(root_seed);
                            }
                        }
                    } else {
                        return None;
                    }
                    delay_password_retry().await;
                }
            }
            _ => self.get_root_seed(prompt).await,
        }
    }
}

/// Signer State
#[derive(derivative::Derivative)]
#[derivative(Clone(bound = ""))]
pub struct State<A>(Arc<Mutex<InnerState<A>>>)
where
    A: Authorizer;

impl<A> State<A>
where
    A: Authorizer,
{
    /// Builds a new [`State`] using `config` and `authorizer`.
    #[inline]
    pub fn new(config: Config, authorizer: A) -> Self {
        Self(Arc::new(Mutex::new(InnerState::new(config, authorizer))))
    }

    /// Returns the server configuration for `self`.
    #[inline]
    async fn config(&self) -> Config {
        // TODO: Consider removing this clone, if possible.
        self.0.lock().await.config.clone()
    }

    /// Returns the stored root seed if it exists, otherwise, gets the password from the user
    /// and tries to decrypt the root seed.
    #[inline]
    async fn get_root_seed(&self, prompt: A::Prompt) -> Option<RootSeed> {
        self.0.lock().await.get_root_seed(prompt).await
    }

    /// Returns the currently stored root seed if it matches the one returned by the user after
    /// prompting.
    #[inline]
    async fn check_root_seed(&self, prompt: A::Prompt) -> Option<RootSeed> {
        self.0.lock().await.check_root_seed(prompt).await
    }
}

/// Server Request Type
pub type Request<A> = ServerRequest<State<A>>;

/// Signer Service
pub struct Service<A>(Server<State<A>>)
where
    A: Authorizer;

impl<A> Service<A>
where
    A: 'static + Authorizer<Prompt = Prompt> + Send + Sync,
    A::Message: Send,
    A::Error: Send,
{
    /// Builds a new [`Service`] from `config` and `authorizer`.
    #[inline]
    pub fn build(config: Config, authorizer: A) -> Self {
        let cors = CorsMiddleware::new()
            .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
            .allow_origin(Origin::from(config.origin_url.as_str()))
            .allow_credentials(false);
        let mut server = Server::with_state(State::new(config, authorizer));
        server.with(cors);
        server.at("/ws").get(WebSocket::new(move |request, mut stream| {
            async move {

                while let Some(Ok(Message::Text(input))) = stream.next().await {
                    //Self::log(format!("[INFO]: Message {input}")).await?;
                    let message: SignerMessage = match serde_json::from_str(&input) {
                        Ok(msg) => msg,
                        Err(e) => {
                            Self::log(format!("[ERROR]: Bad message from client {e:?}")).await?;
                            stream.send_json(&ServerMessage::BadJson).await?;
                            continue;
                        }
                    };

                    match message {
                        SignerMessage::Version => {
                            Self::log(String::from("REQUEST: version")).await?;
                            stream.send_json(&VersionMessage::default()).await?;
                        }
                        SignerMessage::RecoverAccount(acct) => {
                            // TODO: need a decoding of acct here?
                            let response = Self::recover_account(acct, &request.state()).await?;
                            stream.send_json(&response).await?;
                        }
                        SignerMessage::DeriveShieldedAddress(body) => {
                            // TODO: need a decoding of acct here?
                            let response = Self::derive_shielded_address(body, &request.state()).await?;
                            stream.send_json(&response).await?;
                        }
                        SignerMessage::GenerateAsset(body) => {
                            let response = Self::generate_asset(body, &request.state()).await?;
                            stream.send_json(&response).await?;
                        }
                        SignerMessage::GenerateMintData(body) => {
                            let response = Self::mint(body, &request.state()).await?;
                            stream.send_json(&response).await?;
                        }
                        SignerMessage::GeneratePrivateTransferData(body) => {
                            let response = Self::private_transfer(body, &request.state()).await?;
                            stream.send_json(&response).await?;
                        }
                        SignerMessage::GenerateReclaimData(body) => {
                            let response = Self::reclaim(body, &request.state()).await?;
                            stream.send_json(&response).await?;
                        }
                    }
                }
                Ok(())
            }
        }));
        Self(server)
    }

    /// Starts the service.
    #[inline]
    pub async fn serve(self) -> io::Result<()> {
        Self::log(String::from("[INFO]: Starting Service ...")).await?;
        let service_url = {
            let state = &mut *self.0.state().0.lock().await;

            Self::log(String::from("Setting up configuration: ")).await?;
            state.config.setup().await?;

            Self::log(String::from("Setting up authorizer: ")).await?;
            state.authorizer.setup(&state.config).await;

            Self::log(String::from("Checking password: ")).await?;
            loop {
                if state.check_starting_password().await {
                    state.authorizer.success(Default::default()).await;
                    break;
                }
                delay_password_retry().await;
            }

            state.config.service_url.clone()
        };
        Self::log(String::from("DONE. Listening ...")).await?;
        self.0.listen(service_url).await
    }

    /// Returns a reference to the internal state of the service.
    #[inline]
    pub fn state(&self) -> &State<A> {
        self.0.state()
    }

    /// Runs an account recovery for the given `request`.
    #[inline]
    async fn recover_account(body: Vec<u8>, state: &State<A>) -> ServerResult<RecoverAccountMessage> {
        let now = Instant::now();
        let params = ensure!(RecoverAccountParams::decode(&mut body.as_slice()))?;
        Self::log(String::from("REQUEST: RecoverAccountParams { ... }")).await?;
        let root_seed = ensure!(state.get_root_seed(Prompt::RecoverAccount).await.ok_or(()))?;
        println!("TJDEBUG asdf");
        let recovered_account =
            manta_api::recover_account(params, root_seed.expose_secret()).encode();
        println!("TJDEBUG anddere");
        Self::log(format!(
            "RESPONSE: {:?} ({} ms)",
            recovered_account,
            now.elapsed().as_millis()
        ))
        .await?;
        Ok(RecoverAccountMessage::new(recovered_account))
    }

    /// Generates a new derived shielded address for the given `request`.
    #[inline]
    async fn derive_shielded_address(body: Vec<u8>, state: &State<A>) -> ServerResult<ShieldedAddressMessage> {
        let now = Instant::now();
        let params = ensure!(DeriveShieldedAddressParams::decode(&mut body.as_slice(),))?;
        Self::log(format!("REQUEST: {:?}", params)).await?;
        let root_seed = ensure!(state
            .get_root_seed(Prompt::DeriveShieldedAddress)
            .await
            .ok_or(()))?;
        let mut address = Vec::new();
        ensure!(
            manta_api::derive_shielded_address(params, root_seed.expose_secret())
                .serialize(&mut address)
        )?;
        Self::log(format!(
            "RESPONSE: {:?} ({} ms)",
            address,
            now.elapsed().as_millis()
        ))
        .await?;
        Ok(ShieldedAddressMessage::new(address))
    }

    /// Generates an asset for the given `request`.
    #[inline]
    async fn generate_asset(body: Vec<u8>, state: &State<A>) -> ServerResult<AssetMessage> {
        let now = Instant::now();
        let params = ensure!(GenerateAssetParams::decode(&mut body.as_slice()))?;
        Self::log(format!("REQUEST: {:?}", params)).await?;
        let root_seed = ensure!(state.get_root_seed(Prompt::GenerateAsset).await.ok_or(()))?;
        let asset =
            manta_api::generate_signer_input_asset(params, root_seed.expose_secret()).encode();
        Self::log(format!(
            "RESPONSE: {:?} ({} ms)",
            asset,
            now.elapsed().as_millis()
        ))
        .await?;
        Ok(AssetMessage::new(asset))
    }

    /// Generates mint data for the given `request`.
    #[inline]
    async fn mint(body: Vec<u8>, state: &State<A>) -> ServerResult<MintMessage> {
        let now = Instant::now();
        let params = ensure!(GenerateAssetParams::decode(&mut body.as_slice()))?;
        Self::log(format!("REQUEST: {:?}", params)).await?;
        let root_seed = ensure!(state.get_root_seed(Prompt::Mint).await.ok_or(()))?;
        let mut mint_data = Vec::new();
        ensure!(
            manta_api::generate_mint_data(params, root_seed.expose_secret())
                .serialize(&mut mint_data)
        )?;
        Self::log(format!(
            "RESPONSE: {:?} ({} ms)",
            mint_data,
            now.elapsed().as_millis()
        ))
        .await?;
        Ok(MintMessage::new(mint_data))
    }

    /// Generates private transfer data for the given `request`.
    #[inline]
    async fn private_transfer(body: Vec<u8>, state: &State<A>) -> ServerResult<PrivateTransferMessage> {
        let now = Instant::now();
        let params = ensure!(GeneratePrivateTransferBatchParams::decode(
            &mut body.as_slice()
        ))?;
        Self::log(format!("REQUEST: {:?}", params)).await?;
        let root_seed = ensure!(state.check_root_seed(Prompt::from(&params)).await.ok_or(()))?;
        let private_transfer_data = batch_generate_private_transfer_data(
            params,
            *root_seed.expose_secret(),
            state.config().await.private_transfer_proving_key_path(),
            Self::rng,
        )
        .await
        .encode();
        Self::log(format!(
            "RESPONSE: {:?} ({} ms)",
            private_transfer_data,
            now.elapsed().as_millis()
        ))
        .await?;
        Ok(PrivateTransferMessage::new(private_transfer_data))
    }

    /// Generates reclaim data for the given `request`.
    #[inline]
    async fn reclaim(body: Vec<u8>, state: &State<A>) -> ServerResult<ReclaimMessage> {
        let now = Instant::now();
        let params = ensure!(GenerateReclaimBatchParams::decode(&mut body.as_slice()))?;
        Self::log(format!("REQUEST: {:?}", params)).await?;
        let root_seed = ensure!(state.check_root_seed(Prompt::from(&params)).await.ok_or(()))?;
        let config = state.config().await;
        let reclaim_data = batch_generate_reclaim_data(
            params,
            *root_seed.expose_secret(),
            config.private_transfer_proving_key_path(),
            config.reclaim_proving_key_path(),
            Self::rng,
        )
        .await
        .encode();
        Self::log(format!(
            "RESPONSE: {:?} ({} ms)",
            reclaim_data,
            now.elapsed().as_millis()
        ))
        .await?;
        Ok(ReclaimMessage::new(reclaim_data))
    }

    /// Logs the string to the console.
    #[inline]
    async fn log(string: String) -> io::Result<()> {
        let mut stdout = io::stdout();
        stdout.write_all(string.as_bytes()).await?;
        stdout.write_all(b"\n\n").await
    }

    /// Samples a new RNG for generating ZKPs.
    #[inline]
    fn rng() -> ChaCha20Rng {
        ChaCha20Rng::from_rng(thread_rng()).expect("Unable to sample RNG.")
    }
}

// TODO: check the encodings for these...
/// Signer message
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SignerMessage {
    /// Version
    Version,
    /// Recover
    RecoverAccount(Vec<u8>),
    /// ShieldedAddress
    DeriveShieldedAddress(Vec<u8>),
    /// Asset
    GenerateAsset(Vec<u8>),
    /// MintData
    GenerateMintData(Vec<u8>),
    /// PrivateTransfer
    GeneratePrivateTransferData(Vec<u8>),
    /// Reclaim
    GenerateReclaimData(Vec<u8>),
}

/// Server message
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ServerMessage {
    /// Bad json
    BadJson
}

/// Version Message
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VersionMessage {
    /// Version
    pub version: &'static str,
}

impl Default for VersionMessage {
    /// Builds a default [`VersionMessage`].
    fn default() -> Self {
        Self { version: VERSION }
    }
}

/// Shielded Address Message
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShieldedAddressMessage {
    /// Address
    pub address: Vec<u8>,

    /// Version
    pub version: &'static str,
}

impl ShieldedAddressMessage {
    /// Builds a new [`ShieldedAddressMessage`].
    #[inline]
    pub fn new(address: Vec<u8>) -> Self {
        Self {
            address,
            version: VERSION,
        }
    }
}
/// Recover Account Message
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RecoverAccountMessage {
    /// Recovered Account
    pub recovered_account: Vec<u8>,

    /// Version
    pub version: &'static str,
}

impl RecoverAccountMessage {
    /// Builds a new [`RecoverAccountMessage`].
    #[inline]
    pub fn new(recovered_account: Vec<u8>) -> Self {
        Self {
            recovered_account,
            version: VERSION,
        }
    }
}

/// Asset Message
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetMessage {
    /// Asset
    pub asset: Vec<u8>,

    /// Version
    pub version: &'static str,
}

impl AssetMessage {
    /// Builds a new [`AssetMessage`].
    #[inline]
    pub fn new(asset: Vec<u8>) -> Self {
        Self {
            asset,
            version: VERSION,
        }
    }
}

/// Mint Message
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MintMessage {
    /// Mint Data
    pub mint_data: Vec<u8>,

    /// Version
    pub version: &'static str,
}

impl MintMessage {
    /// Builds a new [`MintMessage`].
    #[inline]
    pub fn new(mint_data: Vec<u8>) -> Self {
        Self {
            mint_data,
            version: VERSION,
        }
    }
}

/// Private Transfer Message
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateTransferMessage {
    /// Private Transfer Data
    pub private_transfer_data: Vec<u8>,

    /// Version
    pub version: &'static str,
}

impl PrivateTransferMessage {
    /// Builds a new [`PrivateTransferMessage`].
    #[inline]
    pub fn new(private_transfer_data: Vec<u8>) -> Self {
        Self {
            private_transfer_data,
            version: VERSION,
        }
    }
}

/// Reclaim Message
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReclaimMessage {
    /// Reclaim Data
    pub reclaim_data: Vec<u8>,

    /// Version
    pub version: &'static str,
}

impl ReclaimMessage {
    /// Builds a new [`ReclaimMessage`].
    #[inline]
    pub fn new(reclaim_data: Vec<u8>) -> Self {
        Self {
            reclaim_data,
            version: VERSION,
        }
    }
}
