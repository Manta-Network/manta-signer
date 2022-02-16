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

//! Manta Signer Service Implementation

use crate::{
    config::Config,
    secret::{Argon2, Authorizer, ExposeSecret, PasswordHash},
};
use core::future::Future;
use core::time::Duration;
use manta_accounting::fs::{cocoon::File, File as _, SaveError};
use manta_pay::{
    config::{ReceivingKey, Transaction},
    signer::{
        base::Signer, ReceivingKeyRequest, SignError, SignResponse, SyncError, SyncRequest,
        SyncResponse,
    },
};
use manta_util::{
    from_variant_impl,
    serde::{de::DeserializeOwned, Serialize},
};
use parking_lot::Mutex;
use std::{
    io,
    net::{AddrParseError, SocketAddr},
    sync::Arc,
};
use tokio::{
    fs,
    task::{self, JoinError},
};
use warp::{
    http::{Method, StatusCode},
    reply::{self, Json, Reply},
    Filter, Rejection,
};

/// Password Retry Interval
pub const PASSWORD_RETRY_INTERVAL: Duration = Duration::from_millis(1000);

/// Sets the task to sleep to delay password retry.
#[inline]
async fn delay_password_retry() {
    tokio::time::sleep(PASSWORD_RETRY_INTERVAL).await;
}

/// Service Error
#[derive(Debug)]
pub enum Error {
    /// Address Parsing Error
    AddrParseError(AddrParseError),

    /// Runtime Join Error
    JoinError(JoinError),

    /// Failed to Load SDK Parameters
    ParameterLoadingError,

    /// Save Error
    SaveError(SaveError<File>),

    /// Generic I/O Error
    Io(io::Error),
}

from_variant_impl!(Error, AddrParseError, AddrParseError);
from_variant_impl!(Error, JoinError, JoinError);
from_variant_impl!(Error, SaveError, SaveError<File>);
from_variant_impl!(Error, Io, io::Error);

/// Inner State
struct InnerState<A>
where
    A: Authorizer,
{
    /// Configuration
    config: Config,

    /// Authorizer
    authorizer: A,

    /// Password Hash
    password_hash: PasswordHash<Argon2>,

    /// Signer
    signer: Signer,
}

impl<A> InnerState<A>
where
    A: Authorizer,
{
    /// Builds a new [`InnerState`] from `config`, `authorizer`, `password_hash`, and `signer`.
    #[inline]
    fn new(
        config: Config,
        authorizer: A,
        password_hash: PasswordHash<Argon2>,
        signer: Signer,
    ) -> Self {
        Self {
            config,
            authorizer,
            password_hash,
            signer,
        }
    }

    /// Checks that the authorizer's password matches the known password by sending the `prompt`.
    #[inline]
    async fn check_password<T>(&mut self, prompt: &T) -> bool
    where
        T: Serialize,
    {
        self.authorizer.wake(prompt).await;
        loop {
            if let Some(password) = self.authorizer.password().await.known() {
                if self.password_hash.verify(password.expose_secret()).is_ok() {
                    self.authorizer.sleep().await;
                    return true;
                }
            } else {
                return false;
            }
            delay_password_retry().await;
        }
    }
}

/// Signer Server State
#[derive(derivative::Derivative)]
#[derivative(Clone(bound = ""))]
pub struct State<A>(Arc<Mutex<InnerState<A>>>)
where
    A: Authorizer;

impl<A> State<A>
where
    A: Authorizer,
{
    /// Builds a new [`State`] from `config` and `authorizer`.
    #[inline]
    async fn build(config: Config, mut authorizer: A) -> Result<Self, Error> {
        let parameters = task::spawn_blocking(crate::parameters::load)
            .await?
            .ok_or(Error::ParameterLoadingError)?;
        authorizer.setup(&config).await;
        let (password_hash, signer) = loop {
            if let Some(password) = authorizer.password().await.known() {
                let password = password.expose_secret();
                if let Ok(password_hash) = PasswordHash::from_default(password) {
                    if let Ok(state) = File::load(&config.data_path, &password_hash.as_bytes()) {
                        authorizer.sleep().await;
                        break (password_hash, Signer::from_parts(parameters, state));
                    }
                }
            }
            delay_password_retry().await;
        };
        Ok(Self(Arc::new(Mutex::new(InnerState::new(
            config,
            authorizer,
            password_hash,
            signer,
        )))))
    }

    /// Builds an endpoint for `command` to run `f` as the action.
    #[inline]
    fn endpoint<T, R, Fut, F>(
        self,
        command: &'static str,
        f: F,
    ) -> impl Clone + Filter<Extract = (reply::Response,), Error = Rejection>
    where
        T: DeserializeOwned + Send,
        R: Serialize + Send,
        Fut: Future<Output = Option<R>> + Send,
        F: Clone + Send + Sync + Fn(Self, T) -> Fut,
    {
        warp::path(command)
            .map(move || self.clone())
            .and(warp::body::content_length_limit(1024 * 128))
            .and(warp::body::json())
            .then(f)
            .then(move |response: Option<_>| async {
                Response(response.map(|r| warp::reply::json(&r))).into()
            })
    }

    /// Saves the signer state to disk.
    #[inline]
    async fn save(self) -> Result<(), Error> {
        let path = { self.0.lock().config.data_path.clone() };
        let backup = path.with_extension("backup");
        fs::rename(&path, &backup).await?;
        task::spawn_blocking(move || {
            let lock = self.0.lock();
            File::save(path, &lock.password_hash.as_bytes(), lock.signer.state())
        })
        .await??;
        fs::remove_file(backup).await?;
        Ok(())
    }

    /// Returns the [`crate::VERSION`] string to the client.
    #[inline]
    async fn version(self, request: ()) -> Option<&'static str> {
        let _ = (self, request);
        Some(crate::VERSION)
    }

    /// Runs the synchronization protocol on the signer.
    #[inline]
    async fn sync(self, request: SyncRequest) -> Option<Result<SyncResponse, SyncError>> {
        Some(self.0.lock().signer.sync(request))
    }

    /// Runs the transaction signing protocol on the signer.
    #[inline]
    async fn sign(self, transaction: Transaction) -> Option<Result<SignResponse, SignError>> {
        // TODO: authorizer.prompt(transaction)
        // let _ = self.signer.sign(transaction);
        todo!()
    }

    /// Runs the receiving key sampling protocol on the signer.
    #[inline]
    async fn receiving_keys(self, request: ReceivingKeyRequest) -> Option<Vec<ReceivingKey>> {
        Some(self.0.lock().signer.receiving_keys(request))
    }
}

/// HTTP Response
#[derive(Default)]
pub struct Response(Option<Json>);

impl From<Response> for reply::Response {
    #[inline]
    fn from(response: Response) -> Self {
        match response.0 {
            Some(json) => json.into_response(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

/// Serves the signer server with `config` and `authorizer`.
#[inline]
pub async fn serve<A>(config: Config, authorizer: A) -> Result<(), Error>
where
    A: Authorizer,
{
    let socket_address = config.service_url.parse::<SocketAddr>()?;
    let cors = warp::cors()
        .allow_origin(config.origin_url.as_str())
        .allow_methods(&[Method::GET, Method::POST])
        .allow_credentials(false)
        .build();
    let state = State::build(config, authorizer).await?;
    let api = warp::get()
        .and(state.clone().endpoint("version", State::version))
        .or(warp::post().and(state.clone().endpoint("sync", State::sync)))
        .or(warp::get().and(state.clone().endpoint("sign", State::sign)))
        .or(warp::post().and(
            state
                .clone()
                .endpoint("receivingKeys", State::receiving_keys),
        ))
        .with(cors);
    warp::serve(api).run(socket_address).await;
    Ok(())
}
