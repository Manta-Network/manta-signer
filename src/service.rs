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
    http,
    log::{info, trace, warn},
    secret::{Argon2, Authorizer, ExposeSecret, PasswordHash, SecretString},
};
use alloc::sync::Arc;
use core::{
    fmt::{self, Display},
    time::Duration,
};
use http_types::headers::HeaderValue;
use manta_accounting::{
    fs::{cocoon::File, File as _, SaveError},
    key::HierarchicalKeyDerivationScheme,
    transfer::canonical::TransferShape,
    wallet::signer::NetworkType
};
use manta_pay::{
    key::{Mnemonic, TestnetKeySecret},
    signer::base::{
        HierarchicalKeyDerivationFunction, Signer, SignerParameters, SignerState, UtxoAccumulator,
    },
};
use manta_util::{from_variant, serde::Serialize};
use parking_lot::Mutex;
use std::{
    io,
    net::{AddrParseError, SocketAddr},
    path::Path,
};
use tide::{
    security::{CorsMiddleware, Origin},
    StatusCode,
};
use tokio::{
    fs,
    sync::Mutex as AsyncMutex,
    task::{self, JoinError},
};

pub use manta_pay::{
    config::{receiving_key_to_base58, ReceivingKey},
    signer::{
        ReceivingKeyRequest, SignError, SignRequest, SignResponse, SyncError, SyncRequest,
        SyncResponse,
    },
};

use manta_pay_old::signer::base::SignerState as OldSignerState;


/// Password Retry Interval
pub const PASSWORD_RETRY_INTERVAL: Duration = Duration::from_millis(1000);

/// Sets the task to sleep to delay password retry.
#[inline]
pub async fn delay_password_retry() {
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

    /// Authorization Error
    AuthorizationError,
}

from_variant!(Error, AddrParseError, AddrParseError);
from_variant!(Error, JoinError, JoinError);
from_variant!(Error, SaveError, SaveError<File>);
from_variant!(Error, Io, io::Error);

impl From<Error> for tide::Error {
    #[inline]
    fn from(err: Error) -> tide::Error {
        match err {
            Error::AuthorizationError => {
                Self::from_str(StatusCode::Unauthorized, "request was not authorized")
            }
            _ => Self::from_str(
                StatusCode::InternalServerError,
                "unable to complete request",
            ),
        }
    }
}

impl Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AddrParseError(err) => write!(f, "Address Parse Error: {}", err),
            Self::JoinError(err) => write!(f, "Join Error: {}", err),
            Self::ParameterLoadingError => write!(f, "Parameter Loading Error"),
            Self::SaveError(err) => write!(f, "Save Error: {}", err),
            Self::Io(err) => write!(f, "I/O Error: {}", err),
            Self::AuthorizationError => write!(f, "Authorization Error"),
        }
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

    /// Dolphin Signer
    dolphin_signer: Signer,

    /// Calamari Signer
    calamari_signer: Signer,

    /// Manta Signer
    manta_signer: Signer
}

/// Signer Server
#[derive(derivative::Derivative)]
#[derivative(Clone(bound = ""))]
pub struct Server<A>
where
    A: Authorizer,
{
    /// Server State
    state: Arc<Mutex<State>>,

    /// Authorizer
    authorizer: Arc<AsyncMutex<CheckedAuthorizer<A>>>
}

impl<A> Server<A>
where
    A: Authorizer,
{

    /// Builds a new [`Server`] from `config` and `authorizer`.
    #[inline]
    pub async fn build(config: Config, mut authorizer: A) -> Result<Self> {
        info!("building signer server")?;
        info!("loading latest parameters from Manta Parameters")?;
        let data_path = config.data_directory().to_owned();
        let parameters = task::spawn_blocking(move || crate::parameters::load(data_path))
            .await?
            .ok_or(Error::ParameterLoadingError)?;
        info!("setting up configuration")?;
        let data_exists = config.does_data_exist().await?;
        let setup = authorizer.setup(data_exists).await;
        let (password_hash, dolphin_signer, calamari_signer, manta_signer) = match setup {
            Setup::CreateAccount(mnemonic) => loop {
                if let Some((_password, password_hash)) = Self::load_password(&mut authorizer).await
                {
                    info!("creating dolphin state.")?;
                    let dolphin_state = Self::create_state(
                        &config.data_path_dolphin,
                        &password_hash,
                        mnemonic.clone(),
                        parameters.clone(),
                    )
                    .await?;
                    
                    info!("creating calamari state.")?;
                    let calamari_state = Self::create_state(
                        &config.data_path_calamari,
                        &password_hash,
                        mnemonic.clone(),
                        parameters.clone(),
                    )
                    .await?;

                    info!("creating manta state.")?;
                    let manta_state = Self::create_state(
                        &config.data_path_manta,
                        &password_hash,
                        mnemonic.clone(),
                        parameters.clone(),
                    )
                    .await?;

                    break (password_hash, dolphin_state, calamari_state, manta_state);
                }
                delay_password_retry().await;
            },
            Setup::Login => loop {
                if let Some((_, password_hash)) = Self::load_password(&mut authorizer).await {

                    // @TODO: Handle edge case where there is 1 or 2 missing storage.dat files. Need to load the existing one,
                    // get the mnemonic and recreate the missing state files using the mnemonic.

                    // maybe data exists should return true if at least one exists? - then we know which one to load mnemonic from

                    // for now assume all files will always exist.

                    let dolphin_state = 
                    Self::load_state(&config.data_path_dolphin, &password_hash, &mut authorizer)
                    .await?
                    .expect("Unable to get dolphin state");
                    let dolphin_signer = Signer::from_parts(parameters.clone(), dolphin_state);

                    let calamari_state = 
                    Self::load_state(&config.data_path_calamari, &password_hash, &mut authorizer)
                    .await?
                    .expect("Unable to get calamari state");
                    let calamari_signer = Signer::from_parts(parameters.clone(), calamari_state);

                    let manta_state = 
                    Self::load_state(&config.data_path_manta, &password_hash, &mut authorizer)
                    .await?
                    .expect("Unable to get manta state");
                    let manta_signer = Signer::from_parts(parameters.clone(), manta_state);

                    break (password_hash, dolphin_signer, calamari_signer, manta_signer);
                }
                delay_password_retry().await;
            },
        };
        info!("telling authorizer to sleep")?;
        authorizer.sleep().await;
        Ok(Self {
            state: Arc::new(Mutex::new(State { config, dolphin_signer, calamari_signer, manta_signer })),
            authorizer: Arc::new(AsyncMutex::new(CheckedAuthorizer {
                password_hash,
                authorizer,
            })),
        })
    }

    /// Starts the signer server with `config` and `authorizer`.
    #[inline]
    pub async fn start(self) -> Result<()> {
        let config = self.state.lock().config.clone();
        info!("performing service setup with {:#?}", config)?;
        let socket_address = config.service_url.parse::<SocketAddr>()?;
        let cors = CorsMiddleware::new()
            .allow_methods("GET, POST".parse::<HeaderValue>().unwrap())
            .allow_origin(match &config.origin_url {
                Some(origin_url) => Origin::from(origin_url.as_str()),
                _ => Origin::from("*"),
            })
            .allow_credentials(false);
        let mut api = tide::Server::with_state(self);
        api.with(cors);
        api.at("/version")
            .get(|_| http::into_body(Server::<A>::version));
        http::register_post(&mut api, "/sync", Server::sync);
        http::register_post(&mut api, "/sign", Server::sign);
        http::register_post(&mut api, "/receivingKeys", Server::receiving_keys);
        info!("serving signer API at {}", socket_address)?;
        api.listen(socket_address).await?;
        Ok(())
    }

    /// Loads the password from the `authorizer` and compute the password hash.
    #[inline]
    async fn load_password(authorizer: &mut A) -> Option<(SecretString, PasswordHash<Argon2>)> {
        info!("loading password from authorizer").ok()?;
        let password = authorizer.password().await.known()?;
        let password_hash = PasswordHash::from_default(password.expose_secret().as_bytes());
        Some((password, password_hash))
    }

    /// Creates the initial signer state for a new account.
    #[inline]
    async fn create_state(
        data_path: &Path,
        password_hash: &PasswordHash<Argon2>,
        mnemonic: Mnemonic,
        parameters: SignerParameters,
    ) -> Result<Signer> {
        info!("creating signer state")?;
        let state = SignerState::new(
            TestnetKeySecret::new(mnemonic, "")
                .map(HierarchicalKeyDerivationFunction::default()),
            UtxoAccumulator::new(
                task::spawn_blocking(crate::parameters::load_utxo_accumulator_model)
                    .await?
                    .ok_or(Error::ParameterLoadingError)?,
            ),
        );
        info!("saving signer state")?;
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
        authorizer: &mut A
    ) -> Result<Option<SignerState>> {
        info!("loading signer state from disk")?;
        let data_path = data_path.to_owned();
        let password_hash_bytes = password_hash.as_bytes();
        let data_path_copy = data_path.to_owned();
        let password_hash_bytes_copy = password_hash.as_bytes();
        
        if let Ok(state) =
            task::spawn_blocking(move || File::load::<_,SignerState>(&data_path ,&password_hash_bytes)).await? 
        {
            Ok(Some(state))
        }
        else if let Ok(_state ) = 
            task::spawn_blocking(move || File::load::<_,OldSignerState>(&data_path_copy ,&password_hash_bytes_copy)).await?
        {
            // @TODO: Can get rid of this? Since we are using files that are named differently, the old storage.dat will never be
            // used/read again.
            authorizer.delete_old_account();
            Ok(None)
        }
        else
        {
            Ok(None)
        }
    }

    /// Saves the signer state corresponding to `NetworkType` to disk.
    #[inline]
    async fn save(self, network: NetworkType) -> Result<()> {
        info!("starting signer state save to disk for {}", network.display())?;

        let path = match network {
            NetworkType::Dolphin => {
                self.state.lock().config.data_path_dolphin.clone()
            }, 
            NetworkType::Calamari => {
                self.state.lock().config.data_path_calamari.clone()
            },
            NetworkType::Manta => {
                self.state.lock().config.data_path_manta.clone()
            } 
        };

        let backup = path.with_extension("backup");
        fs::rename(&path, &backup).await?;
        let password_hash_bytes = self.authorizer.lock().await.password_hash.as_bytes();
        let network_copy = network.clone();
        task::spawn_blocking(move || {
            let lock = self.state.lock();

            let state_to_save = match network_copy {
                NetworkType::Dolphin => {
                    lock.dolphin_signer.state()
                }, 
                NetworkType::Calamari => {
                    lock.calamari_signer.state()
                },
                NetworkType::Manta => {
                    lock.manta_signer.state()
                } 
            };

            File::save(path, &password_hash_bytes, state_to_save)
        })
        .await??;
        fs::remove_file(backup).await?;
        info!("save complete for {}", network.display())?;
        Ok(())
    }

    /// Returns the [`crate::VERSION`] string to the client.
    #[inline]
    pub async fn version() -> Result<&'static str> {
        trace!("[PING] current signer version: {}", crate::VERSION)?;
        Ok(crate::VERSION)
    }


    /// Runs the synchronization protocol on the signer.
    #[inline]
    pub async fn sync(self, request: SyncRequest) -> Result<Result<SyncResponse, SyncError>> {
        info!("[REQUEST] processing `sync`:  {:?}.", request)?;

        let response = match request.network {
            NetworkType::Dolphin => {
                self.state.lock().dolphin_signer.sync(request)
            }, 
            NetworkType::Calamari => {
                self.state.lock().calamari_signer.sync(request)
            },
            NetworkType::Manta => {
                self.state.lock().manta_signer.sync(request)
            } 
        };
    
        task::spawn(async {
            if self.save(request.network).await.is_err() {
                let _ = warn!("unable to save current signer state");
            }
        });
        info!("[RESPONSE] responding to `sync` with: {:?}.", response)?;
        Ok(response)
    }

    /// Runs the transaction signing protocol on the signer.
    #[inline]
    pub async fn sign(self, request: SignRequest) -> Result<Result<SignResponse, SignError>> {
        info!("[REQUEST] processing `sign`: {:?}.", request)?;
        let SignRequest {
            transaction,
            metadata,
            network
        } = request;
        match transaction.shape() {
            TransferShape::Mint => {
                // NOTE: We skip authorization on mint transactions because they are deposits not
                //       withdrawals from the point of view of the signer. Everything else, by
                //       default, requests authorization.
            }
            _ => {
                info!("[AUTH] asking for transaction authorization")?;
                let summary = metadata
                    .map(|m| transaction.display(&m, &network, receiving_key_to_base58))
                    .unwrap_or_default();
                self.authorizer.lock().await.check(&summary).await?
            }
        }

        let response = match network {
            NetworkType::Dolphin => {
                self.state.lock().dolphin_signer.sign(transaction)
            }, 
            NetworkType::Calamari => {
                self.state.lock().calamari_signer.sign(transaction)
            },
            NetworkType::Manta => {
                self.state.lock().manta_signer.sign(transaction)
            } 
        };
        info!("[RESPONSE] responding to `sign` with: {:?}.", response)?;
        Ok(response)
    }

    /// Gets the mnemonic stored on disk for a specific `NetworkType`.
    #[inline]
    pub async fn get_stored_mnemonic(&mut self, prompt: &String, network: NetworkType) -> Result<Mnemonic> {

        self.authorizer.lock().await.check(prompt).await?;

        let stored_mnemonic = match network {
            NetworkType::Dolphin => {
                self.state.lock().dolphin_signer.state().accounts().keys().base().expose_mnemonic().clone()
            }, 
            NetworkType::Calamari => {
                self.state.lock().calamari_signer.state().accounts().keys().base().expose_mnemonic().clone()
            },
            NetworkType::Manta => {
                self.state.lock().manta_signer.state().accounts().keys().base().expose_mnemonic().clone()
            } 
        };
        Ok(stored_mnemonic)
    }

    /// Runs the receiving key sampling protocol on the signer.
    /// Uses default Dolphin Signer to get receiving keys.
    #[inline]
    pub async fn receiving_keys(self, request: ReceivingKeyRequest) -> Result<Vec<ReceivingKey>> {
        info!("[REQUEST] processing `receivingKeys`: {:?}", request)?;
        let response = self.state.lock().dolphin_signer.receiving_keys(request);
        info!(
            "[RESPONSE] responding to `receivingKeys` with: {:?}",
            response
        )?;
        Ok(response)
    }
}
