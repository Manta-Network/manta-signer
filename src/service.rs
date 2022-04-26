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
    config::{Config, Setup},
    secret::{Argon2, Authorizer, ExposeSecret, PasswordHash, SecretString},
};
use core::{fmt, future::Future, time::Duration};
use http_types::headers::HeaderValue;
use manta_accounting::{
    fs::{cocoon::File, File as _, SaveError},
    key::HierarchicalKeyDerivationScheme,
    transfer::canonical::TransferShape,
};
use manta_pay::{
    config::{receiving_key_to_base58, ReceivingKey},
    key::{Mnemonic, TestnetKeySecret},
    signer::{
        base::{
            HierarchicalKeyDerivationFunction, Signer, SignerParameters, SignerState,
            UtxoAccumulator,
        },
        ReceivingKeyRequest, SignError, SignRequest, SignResponse, SyncError, SyncRequest,
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
    path::Path,
    sync::Arc,
};
use tide::{
    security::{CorsMiddleware, Origin},
    Body, Request, Response, StatusCode,
};
use tokio::{
    fs,
    sync::Mutex as AsyncMutex,
    task::{self, JoinError},
};

/// Password Retry Interval
pub const PASSWORD_RETRY_INTERVAL: Duration = Duration::from_millis(1000);

/// Sets the task to sleep to delay password retry.
#[inline]
async fn delay_password_retry() {
    tokio::time::sleep(PASSWORD_RETRY_INTERVAL).await;
}

/// Log Level
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum LogLevel {
    /// Information
    Info,

    /// Warning
    Warn,
}

impl fmt::Display for LogLevel {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Info => write!(f, "INFO"),
            Self::Warn => write!(f, "WARN"),
        }
    }
}

/// Logs `string` to STDOUT with the current time.
#[cfg(feature = "log")]
#[inline]
async fn log(level: LogLevel, string: impl Into<String>) -> io::Result<()> {
    use tokio::io::AsyncWriteExt;
    tokio::io::stdout()
        .write_all(
            format!(
                "{} [{}]: {}\n",
                level,
                chrono::offset::Utc::now(),
                string.into()
            )
            .as_bytes(),
        )
        .await
}

/// Does not log to STDOUT when logging is not enabled.
#[cfg(not(feature = "log"))]
#[inline]
async fn log(level: LogLevel, string: impl Into<String>) -> io::Result<()> {
    let _ = (level, string);
    Ok(())
}

/// Prints an INFO-level log to STDOUT.
#[inline]
async fn info(string: impl Into<String>) -> io::Result<()> {
    log(LogLevel::Info, string).await
}

/// Prints an WARN-level log to STDOUT.
#[inline]
async fn warn(string: impl Into<String>) -> io::Result<()> {
    log(LogLevel::Warn, string).await
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

    /// Authorization Error
    AuthorizationError,
}

from_variant_impl!(Error, AddrParseError, AddrParseError);
from_variant_impl!(Error, JoinError, JoinError);
from_variant_impl!(Error, SaveError, SaveError<File>);
from_variant_impl!(Error, Io, io::Error);

impl From<Error> for tide::Error {
    #[inline]
    fn from(err: Error) -> tide::Error {
        // TODO: Convert to a more useful error;
        let _ = err;
        Self::from_str(StatusCode::InternalServerError, "")
    }
}

/// Result Type
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Checked Authorizer
struct CheckedAuthorizer<A>
where
    A: Authorizer,
{
    /// Password Hash
    password_hash: PasswordHash<Argon2>,

    /// Authorizer
    authorizer: A,
}

impl<A> CheckedAuthorizer<A>
where
    A: Authorizer,
{
    /// Checks that the authorizer's password matches the known password by sending the `prompt`.
    #[inline]
    async fn check<T>(&mut self, prompt: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.authorizer.wake(prompt).await;
        loop {
            if let Some(password) = self.authorizer.password().await.known() {
                if self
                    .password_hash
                    .verify(password.expose_secret().as_bytes())
                    .is_ok()
                {
                    self.authorizer.sleep().await;
                    return Ok(());
                }
            } else {
                return Err(Error::AuthorizationError);
            }
            delay_password_retry().await;
        }
    }
}

/// State
struct State {
    /// Configuration
    config: Config,

    /// Signer
    signer: Signer,
}

/// Signer Server
#[derive(derivative::Derivative)]
#[derivative(Clone(bound = ""))]
struct Server<A>
where
    A: Authorizer,
{
    /// Server State
    state: Arc<Mutex<State>>,

    /// Authorizer
    authorizer: Arc<AsyncMutex<CheckedAuthorizer<A>>>,
}

impl<A> Server<A>
where
    A: Authorizer,
{
    /// Builds a new [`Server`] from `config` and `authorizer`.
    #[inline]
    async fn build(config: Config, mut authorizer: A) -> Result<Self> {
        info("building signer server").await?;
        info("loading latest parameters from Manta SDK").await?;
        let data_path = config.data_directory().to_owned();
        let parameters = task::spawn_blocking(move || crate::parameters::load(data_path))
            .await?
            .ok_or(Error::ParameterLoadingError)?;
        info("setting up configuration").await?;
        let setup = config.setup().await?;
        authorizer.setup(&setup).await;
        let (password_hash, signer) = match setup {
            Setup::CreateAccount(mnemonic) => loop {
                if let Some((password, password_hash)) = Self::load_password(&mut authorizer).await
                {
                    let state = Self::create_state(
                        &config.data_path,
                        &password,
                        &password_hash,
                        mnemonic,
                        parameters,
                    )
                    .await?;
                    break (password_hash, state);
                }
                delay_password_retry().await;
            },
            Setup::Login => loop {
                if let Some((_, password_hash)) = Self::load_password(&mut authorizer).await {
                    if let Some(state) = Self::load_state(&config.data_path, &password_hash).await?
                    {
                        break (password_hash, Signer::from_parts(parameters, state));
                    }
                }
                delay_password_retry().await;
            },
        };
        info("telling authorizer to sleep").await?;
        authorizer.sleep().await;
        Ok(Self {
            state: Arc::new(Mutex::new(State { config, signer })),
            authorizer: Arc::new(AsyncMutex::new(CheckedAuthorizer {
                password_hash,
                authorizer,
            })),
        })
    }

    /// Loads the password from the `authorizer` and compute the password hash.
    #[inline]
    async fn load_password(authorizer: &mut A) -> Option<(SecretString, PasswordHash<Argon2>)> {
        info("loading password from authorizer").await.ok()?;
        let password = authorizer.password().await.known()?;
        let password_hash = PasswordHash::from_default(password.expose_secret().as_bytes());
        Some((password, password_hash))
    }

    /// Creates the initial signer state for a new account.
    #[inline]
    async fn create_state(
        data_path: &Path,
        password: &SecretString,
        password_hash: &PasswordHash<Argon2>,
        mnemonic: Mnemonic,
        parameters: SignerParameters,
    ) -> Result<Signer> {
        info("creating signer state").await?;
        let state = SignerState::new(
            TestnetKeySecret::new(mnemonic, password.expose_secret())
                .map(HierarchicalKeyDerivationFunction::default()),
            UtxoAccumulator::new(
                task::spawn_blocking(crate::parameters::load_utxo_accumulator_model)
                    .await?
                    .ok_or(Error::ParameterLoadingError)?,
            ),
        );
        info("saving signer state").await?;
        let data_path = data_path.to_owned();
        let password_hash_bytes = password_hash.as_bytes();
        let cloned_state = state.clone();
        task::spawn_blocking(move || File::save(&data_path, &password_hash_bytes, cloned_state))
            .await??;
        Ok(Signer::from_parts(parameters, state))
    }

    /// Loads the signer state from the data path.
    #[inline]
    async fn load_state(
        data_path: &Path,
        password_hash: &PasswordHash<Argon2>,
    ) -> Result<Option<SignerState>> {
        info("loading signer state from disk").await?;
        let data_path = data_path.to_owned();
        let password_hash_bytes = password_hash.as_bytes();
        if let Ok(state) =
            task::spawn_blocking(move || File::load(&data_path, &password_hash_bytes)).await?
        {
            Ok(Some(state))
        } else {
            Ok(None)
        }
    }

    /// Executes `f` on the incoming `request`.
    #[inline]
    async fn execute<T, R, F, Fut>(
        mut request: Request<Self>,
        f: F,
    ) -> Result<Response, tide::Error>
    where
        T: DeserializeOwned,
        R: Serialize,
        F: FnOnce(Self, T) -> Fut,
        Fut: Future<Output = Result<R>>,
    {
        let args = request.body_json::<T>().await?;
        into_body(move || async move { f(request.state().clone(), args).await }).await
    }

    /// Saves the signer state to disk.
    #[inline]
    async fn save(self) -> Result<()> {
        info("starting signer state save to disk").await?;
        let path = self.state.lock().config.data_path.clone();
        let backup = path.with_extension("backup");
        fs::rename(&path, &backup).await?;
        let password_hash_bytes = self.authorizer.lock().await.password_hash.as_bytes();
        task::spawn_blocking(move || {
            let lock = self.state.lock();
            File::save(path, &password_hash_bytes, lock.signer.state())
        })
        .await??;
        fs::remove_file(backup).await?;
        info("save complete").await?;
        Ok(())
    }

    /// Returns the [`crate::VERSION`] string to the client.
    #[inline]
    async fn version() -> Result<&'static str> {
        info(format!("[PING] current signer version: {}", crate::VERSION)).await?;
        Ok(crate::VERSION)
    }

    /// Runs the synchronization protocol on the signer.
    #[inline]
    async fn sync(self, request: SyncRequest) -> Result<Result<SyncResponse, SyncError>> {
        info(format!("[REQUEST] processing `sync`:  {:?}.", request)).await?;
        let response = self.state.lock().signer.sync(request);
        info(format!(
            "[RESPONSE] responding to `sync` with: {:?}.",
            response
        ))
        .await?;
        Ok(response)
    }

    /// Runs the transaction signing protocol on the signer.
    #[inline]
    async fn sign(self, request: SignRequest) -> Result<Result<SignResponse, SignError>> {
        info(format!("[REQUEST] processing `sign`: {:?}.", request)).await?;
        let SignRequest {
            transaction,
            metadata,
        } = request;
        match transaction.shape() {
            TransferShape::Mint => {
                // NOTE: We skip authorization on mint transactions because they are deposits not
                //       withdrawals from the point of view of the signer. Everything else, by
                //       default, requests authorization.
            }
            _ => {
                info("[AUTH] asking for transaction authorization").await?;
                let summary = metadata
                    .map(|m| transaction.display(&m, receiving_key_to_base58))
                    .unwrap_or_default();
                self.authorizer.lock().await.check(&summary).await?
            }
        }
        let response = self.state.lock().signer.sign(transaction);
        info(format!(
            "[RESPONSE] responding to `sign` with: {:?}.",
            response
        ))
        .await?;
        Ok(response)
    }

    /// Runs the receiving key sampling protocol on the signer.
    #[inline]
    async fn receiving_keys(self, request: ReceivingKeyRequest) -> Result<Vec<ReceivingKey>> {
        info(format!(
            "[REQUEST] processing `receivingKeys`: {:?}",
            request
        ))
        .await?;
        let response = self.state.lock().signer.receiving_keys(request);
        info(format!(
            "[RESPONSE] responding to `receivingKeys` with: {:?}",
            response
        ))
        .await?;
        Ok(response)
    }
}

/// Generates the JSON body for the output of `f`, returning an HTTP reponse.
#[inline]
async fn into_body<R, F, Fut>(f: F) -> Result<Response, tide::Error>
where
    R: Serialize,
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<R>>,
{
    Ok(Body::from_json(&f().await?)?.into())
}

/// Starts the signer server with `config` and `authorizer`.
#[inline]
pub async fn start<A>(config: Config, authorizer: A) -> Result<()>
where
    A: Authorizer,
{
    info("performing service setup").await?;
    let socket_address = config.service_url.parse::<SocketAddr>()?;
    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST".parse::<HeaderValue>().unwrap())
        .allow_origin(match &config.origin_url {
            Some(origin_url) => Origin::from(origin_url.as_str()),
            _ => Origin::from("*"),
        })
        .allow_credentials(false);
    let mut api = tide::Server::with_state(Server::build(config, authorizer).await?);
    api.with(cors);
    api.at("/version").get(|_| into_body(Server::<A>::version));
    api.at("/sync").post(|r| Server::execute(r, Server::sync));
    api.at("/sign").post(|r| Server::execute(r, Server::sign));
    api.at("/receivingKeys")
        .post(|r| Server::execute(r, Server::receiving_keys));
    info("serving signer API").await?;
    api.listen(socket_address).await?;
    Ok(())
}
