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

//! Manta Signer Secrets

use crate::config::Config;
use async_std::{
    fs::{self, File},
    io::{self, ReadExt, WriteExt},
    path::Path,
};
use bip0039::{Count, Mnemonic};
use cocoon::Cocoon;
use core::convert::TryInto;
use futures::future::BoxFuture;
use manta_api::MantaRootSeed;
use rand::{
    distributions::{DistString, Standard},
    CryptoRng, Rng, RngCore,
};
use serde::Serialize;

pub use cocoon::Error as RootSeedError;
pub use secrecy::{ExposeSecret, Secret, SecretString};
pub use subtle::{Choice, ConstantTimeEq, CtOption};

/// Samples a random password string from `rng`.
#[inline]
pub fn sample_password<R>(rng: &mut R) -> SecretString
where
    R: CryptoRng + RngCore + ?Sized,
{
    let length = rng.gen_range(1..65);
    Secret::new(Standard.sample_string(rng, length))
}

/// Password Secret Wrapper
pub struct Password(CtOption<SecretString>);

impl Password {
    /// Builds a new [`Password`] from `password` if `is_known` evaluates to `true`.
    #[inline]
    pub fn new(password: SecretString, is_known: Choice) -> Self {
        Self(CtOption::new(password, is_known))
    }

    /// Builds a new [`Password`] from `password`.
    #[inline]
    pub fn from_known(password: SecretString) -> Self {
        Self::new(password, 1.into())
    }

    /// Builds a new [`Password`] with a no known value.
    #[inline]
    pub fn from_unknown() -> Self {
        Self::new(Secret::new(String::with_capacity(64)), 0.into())
    }

    /// Returns [`Some`] if `self` represents a known password.
    #[inline]
    pub fn known(self) -> Option<SecretString> {
        self.0.into()
    }
}

impl Default for Password {
    #[inline]
    fn default() -> Self {
        Self::from_unknown()
    }
}

/// Password Future
///
/// This `type` is returned by the [`authorize`] and [`setup`] methods on [`Authorizer`].
/// See their documentation for more.
///
/// [`authorize`]: Authorizer::authorize
/// [`setup`]: Authorizer::setup
pub type PasswordFuture<'t> = BoxFuture<'t, Password>;

/// Authorizer
pub trait Authorizer {
    /// Runs some setup for the authorizer using the `config`, returning a [`Password`] if already
    /// known during setup.
    ///
    /// # Implementation Note
    ///
    /// For custom service implementations, this method should be called before any service is run.
    /// [`Service`] already calls this method internally when running [`Service::serve`].
    ///
    /// [`Service`]: crate::service::Service
    /// [`Service::serve`]: crate::service::Service::serve
    #[inline]
    fn setup<'s>(&'s mut self, config: &'s Config) -> PasswordFuture<'s> {
        let _ = config;
        Box::pin(async move { Password::from_unknown() })
    }

    /// Shows the given `prompt` to the authorizer, requesting their password.
    fn authorize<T>(&mut self, prompt: T) -> PasswordFuture
    where
        T: Serialize;
}

/// Root Seed
#[derive(Clone)]
pub struct RootSeed(Secret<MantaRootSeed>);

impl RootSeed {
    /// Builds a new [`RootSeed`], converting the `seed` into a [`Secret`].
    #[inline]
    fn new_secret(seed: MantaRootSeed) -> Self {
        Self(Secret::new(seed))
    }

    /// Builds a new [`RootSeed`] from a `mnemonic`.
    #[inline]
    pub fn new(mnemonic: &Secret<Mnemonic>) -> Self {
        Self::new_secret(mnemonic.expose_secret().to_seed(""))
    }

    /// Saves `self` to the standard root seed file, encrypting it with `password`.
    #[inline]
    pub async fn save<P>(self, path: P, password: &SecretString) -> Result<(), RootSeedError>
    where
        P: AsRef<Path>,
    {
        let mut data = Vec::new();
        Cocoon::new(password.expose_secret().as_bytes())
            .dump(self.0.expose_secret().to_vec(), &mut data)?;
        File::create(path)
            .await
            .map_err(RootSeedError::Io)?
            .write_all(&data)
            .await
            .map_err(RootSeedError::Io)
    }

    /// Loads `self` from the standard root seed file, decrypting it with `password`.
    #[inline]
    pub async fn load<P>(path: P, password: &SecretString) -> Result<Self, RootSeedError>
    where
        P: AsRef<Path>,
    {
        let mut data = Vec::new();
        File::open(path)
            .await
            .map_err(RootSeedError::Io)?
            .read_to_end(&mut data)
            .await
            .map_err(RootSeedError::Io)?;
        Ok(Self::new_secret(
            Cocoon::new(password.expose_secret().as_bytes())
                .parse(&mut data.as_slice())?
                .try_into()
                .expect("Failed to convert root seed file contents to root seed."),
        ))
    }
}

impl ConstantTimeEq for RootSeed {
    #[inline]
    fn ct_eq(&self, rhs: &Self) -> Choice {
        self.expose_secret().ct_eq(rhs.expose_secret())
    }
}

impl ExposeSecret<MantaRootSeed> for RootSeed {
    #[inline]
    fn expose_secret(&self) -> &MantaRootSeed {
        self.0.expose_secret()
    }
}

impl Eq for RootSeed {}

impl PartialEq for RootSeed {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.ct_eq(rhs).into()
    }
}

/// Checks if a root seed exists at the canonical file path.
#[inline]
pub async fn account_exists<P>(path: P) -> io::Result<bool>
where
    P: AsRef<Path>,
{
    Ok(fs::metadata(path).await?.is_file())
}

/// Creates a new account by building and saving a new root seed from the given `password`.
#[inline]
pub async fn create_account<P>(
    path: P,
    password: &SecretString,
) -> Result<Secret<Mnemonic>, RootSeedError>
where
    P: AsRef<Path>,
{
    let mnemonic = Secret::new(Mnemonic::generate(Count::Words12));
    RootSeed::new(&mnemonic).save(path, password).await?;
    Ok(mnemonic)
}
