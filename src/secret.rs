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

// FIXME: Secure passwords.

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
use serde::Serialize;

pub use cocoon::Error as RootSeedError;

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

/// Authorization Future
///
/// This `type` is returned by the [`authorize`] method on [`Authorizer`]. See its documentation for
/// more.
pub type Authorization<'t> = BoxFuture<'t, Option<Password>>;

/// Authorizer
pub trait Authorizer {
    /// Shows the given `prompt` to the authorizer, requesting their password, returning
    /// `None` if the password future failed.
    fn authorize<T>(&mut self, prompt: T) -> Authorization
    where
        T: Serialize;
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
    pub async fn save<P>(self, path: P, password: String) -> Result<(), RootSeedError>
    where
        P: AsRef<Path>,
    {
        let mut data = Vec::new();
        Cocoon::new(password.as_bytes()).dump(self.0.to_vec(), &mut data)?;
        File::create(path)
            .await
            .map_err(RootSeedError::Io)?
            .write_all(&data)
            .await
            .map_err(RootSeedError::Io)
    }

    /// Loads `self` from the standard root seed file, decrypting it with `password`.
    #[inline]
    pub async fn load<P>(path: P, password: String) -> Result<Self, RootSeedError>
    where
        P: AsRef<Path>,
    {
        let mut data = Vec::new();
        File::open(path)
            .await
            .map_err(RootSeedError::Io)?
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
pub async fn account_exists<P>(path: P) -> io::Result<bool>
where
    P: AsRef<Path>,
{
    Ok(fs::metadata(path).await?.is_file())
}

/// Creates a new account by building and saving a new root seed from the given `password`.
#[inline]
pub async fn create_account<P>(path: P, password: String) -> Result<Mnemonic, RootSeedError>
where
    P: AsRef<Path>,
{
    let mnemonic = Mnemonic::generate(Count::Words12);
    RootSeed::new(&mnemonic).save(path, password).await?;
    Ok(mnemonic)
}
