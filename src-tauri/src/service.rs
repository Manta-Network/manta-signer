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

//! Manta Signer Service

use crate::{
    batching::{batch_generate_private_transfer_data, batch_generate_reclaim_data},
    secret::{Password, RootSeed},
};
use async_std::io;
use codec::{Decode, Encode};
use futures::future::BoxFuture;
use manta_api::{
    DeriveShieldedAddressParams, GenerateAssetParams, GeneratePrivateTransferBatchParams,
    GenerateReclaimBatchParams, RecoverAccountParams,
};
use manta_crypto::MantaSerDes;
use rand::{thread_rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::async_runtime::Mutex;
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

/// Authorizer
pub trait Authorizer {
    /// Shows the given `prompt` to the authorizer, requesting their password, returning
    /// `None` if the password future failed.
    fn authorize<T>(&mut self, prompt: T) -> BoxFuture<'_, Option<Password>>
    where
        T: Serialize;
}

/// Inner State
struct InnerState<A>
where
    A: Authorizer,
{
    /// Authorizer
    authorizer: A,

    /// Current Root Seed
    root_seed: Option<RootSeed>,
}

impl<A> InnerState<A>
where
    A: Authorizer,
{
    /// Builds a new [`InnerState`] from `authorizer`.
    #[inline]
    fn new(authorizer: A) -> Self {
        Self {
            authorizer,
            root_seed: None,
        }
    }

    /// Returns the password from the user, prompted with `prompt`.
    #[inline]
    async fn authorize<T>(&mut self, prompt: T) -> Option<Password>
    where
        T: Serialize,
    {
        self.authorizer.authorize(prompt).await
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
#[derive(derivative::Derivative)]
#[derivative(Clone(bound = ""))]
pub struct State<A>(Arc<Mutex<InnerState<A>>>)
where
    A: Authorizer;

impl<A> State<A>
where
    A: Authorizer,
{
    /// Builds a new [`State`] using `authorizer`.
    #[inline]
    pub fn new(authorizer: A) -> Self {
        Self(Arc::new(Mutex::new(InnerState::new(authorizer))))
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
pub type Request<A> = ServerRequest<State<A>>;

/// Signer Service
pub struct Service<A>(Server<State<A>>)
where
    A: Authorizer;

impl<A> Service<A>
where
    A: 'static + Authorizer + Send,
{
    /// Builds a new [`Service`] from `authorizer`.
    #[inline]
    pub fn build(authorizer: A) -> Self {
        let mut server = Server::with_state(State::new(authorizer));
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
        L: ToListener<State<A>>,
    {
        self.0.listen(listener).await
    }

    /// Sends a heartbeat to the client.
    #[inline]
    async fn heartbeat(request: Request<A>) -> ServerResult<String> {
        let _ = request;
        Ok(String::from("heartbeat"))
    }

    /// Runs an account recovery for the given `request`.
    #[inline]
    async fn recover_account(mut request: Request<A>) -> ServerResult {
        let (body, state) = Self::process(&mut request).await?;
        let params = ensure!(RecoverAccountParams::decode(&mut body.as_slice()))?;
        let root_seed = ensure!(state.get_root_seed("recover_account").await.ok_or(()))?;
        let recovered_account = manta_api::recover_account(params, &root_seed.0).encode();
        Ok(Body::from_json(&RecoverAccountMessage::new(recovered_account))?.into())
    }

    /// Generates a new derived shielded address for the given `request`.
    #[inline]
    async fn derive_shielded_address(mut request: Request<A>) -> ServerResult {
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
    async fn generate_asset(mut request: Request<A>) -> ServerResult {
        let (body, state) = Self::process(&mut request).await?;
        let params = ensure!(GenerateAssetParams::decode(&mut body.as_slice()))?;
        let root_seed = ensure!(state.get_root_seed("generate_asset").await.ok_or(()))?;
        let asset = manta_api::generate_signer_input_asset(params, &root_seed.0).encode();
        Ok(Body::from_json(&AssetMessage::new(asset))?.into())
    }

    /// Generates mint data for the given `request`.
    #[inline]
    async fn mint(mut request: Request<A>) -> ServerResult {
        let (body, state) = Self::process(&mut request).await?;
        let params = ensure!(GenerateAssetParams::decode(&mut body.as_slice()))?;
        let root_seed = ensure!(state.get_root_seed("mint").await.ok_or(()))?;
        let mut mint_data = Vec::new();
        ensure!(manta_api::generate_mint_data(params, &root_seed.0).serialize(&mut mint_data))?;
        Ok(Body::from_json(&MintMessage::new(mint_data))?.into())
    }

    /// Generates private transfer data for the given `request`.
    #[inline]
    async fn private_transfer(mut request: Request<A>) -> ServerResult {
        let (body, state) = Self::process(&mut request).await?;
        let params = ensure!(GeneratePrivateTransferBatchParams::decode(
            &mut body.as_slice()
        ))?;
        let root_seed = ensure!(state.check_root_seed("private_transfer").await.ok_or(()))?;
        let mut rng = ChaCha20Rng::from_rng(thread_rng()).expect("Unable to sample RNG.");
        let private_transfer_data =
            batch_generate_private_transfer_data(params, &root_seed.0, "transfer_pk.bin", &mut rng)
                .await
                .encode();
        Ok(Body::from_json(&PrivateTransferMessage::new(private_transfer_data))?.into())
    }

    /// Generates reclaim data for the given `request`.
    #[inline]
    async fn reclaim(mut request: Request<A>) -> ServerResult {
        let (body, state) = Self::process(&mut request).await?;
        let params = ensure!(GenerateReclaimBatchParams::decode(&mut body.as_slice()))?;
        let root_seed = ensure!(state.check_root_seed("private_transfer").await.ok_or(()))?;
        let mut rng = ChaCha20Rng::from_rng(thread_rng()).expect("Unable to sample RNG.");
        let reclaim_data = batch_generate_reclaim_data(
            params,
            &root_seed.0,
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
    async fn process(request: &mut Request<A>) -> ServerResult<(Vec<u8>, &State<A>)> {
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
