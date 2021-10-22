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

use async_std::{
    fs::{self, File},
    io::{self, ReadExt, WriteExt},
};
use bip0039::{Count, Mnemonic};
use cocoon::{Cocoon, Error as CocoonError};
use core::convert::TryInto;
use manta_api::MantaRootSeed;
use std::sync::Arc;
use tauri::async_runtime::{channel, Receiver, RwLock, Sender};

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
