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
use core::future::{self, Future};
use core::time::Duration;
use manta_pay::{
    config::{MultiProvingContext, Parameters, ProvingContext, ReceivingKey, Transaction},
    signer::{
        base::{Signer, SignerParameters, SignerState},
        ReceivingKeyRequest, SignError, SignResponse, SyncError, SyncRequest, SyncResponse,
    },
};
use manta_util::{
    from_variant_impl,
    serde::{de::DeserializeOwned, Serialize},
};
use parking_lot::Mutex;
use std::{
    net::{AddrParseError, SocketAddr},
    sync::Arc,
};
use tokio::task::{self, JoinError};
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
}

from_variant_impl!(Error, AddrParseError, AddrParseError);
from_variant_impl!(Error, JoinError, JoinError);

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
    ///
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

    ///
    #[inline]
    async fn check_password(&mut self, prompt: A::Prompt) -> bool {
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

    ///
    #[inline]
    fn sync(&mut self, request: SyncRequest) -> Result<SyncResponse, SyncError> {
        self.signer.sync(request)
    }

    ///
    #[inline]
    fn sign(&mut self, transaction: Transaction) -> Result<SignResponse, SignError> {
        self.signer.sign(transaction)
    }

    ///
    #[inline]
    fn receiving_keys(&mut self, request: ReceivingKeyRequest) -> Vec<ReceivingKey> {
        self.signer.receiving_keys(request)
    }
}

/// Signer Server State
#[derive(derivative::Derivative)]
#[derivative(Clone(bound = ""))]
pub struct State<A>(Arc<Mutex<InnerState<A>>>)
where
    A: Authorizer + Send;

impl<A> State<A>
where
    A: Authorizer + Send,
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
                    if let Ok(state) = SignerState::load(&config.data_path, password) {
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

    /// Returns the [`crate::VERSION`] string to the client.
    #[inline]
    async fn version(self, request: ()) -> Option<&'static str> {
        let _ = (self, request);
        Some(crate::VERSION)
    }

    ///
    #[inline]
    async fn sync(self, request: SyncRequest) -> Option<Result<SyncResponse, SyncError>> {
        future::ready({ Some(self.0.lock().sync(request)) }).await
    }

    ///
    #[inline]
    async fn sign(self, transaction: Transaction) -> Option<Result<SignResponse, SignError>> {
        // TODO: authorizer.prompt(transaction)
        // let _ = self.signer.sign(transaction);
        todo!()
    }

    ///
    #[inline]
    async fn receiving_keys(self, request: ReceivingKeyRequest) -> Option<Vec<ReceivingKey>> {
        future::ready({ Some(self.0.lock().receiving_keys(request)) }).await
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

///
#[inline]
pub async fn serve<A>(config: Config, authorizer: A) -> Result<(), Error>
where
    A: 'static + Authorizer + Send,
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
