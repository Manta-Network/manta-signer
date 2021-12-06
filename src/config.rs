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

//! Manta Signer Configuration

use async_std::{fs, io};
use std::path::{Path, PathBuf};

/// Manta Path Identifier
const PATH_IDENTIFIER: &str = "manta-signer";

/// Pushes the [`PATH_IDENTIFIER`] to the end of the given `path` if it exists.
#[inline]
fn directory(path: Option<PathBuf>) -> Option<PathBuf> {
    path.map(move |mut p| {
        p.push(PATH_IDENTIFIER);
        p
    })
}

/// Pushes the [`PATH_IDENTIFIER`] to the end of the given `path` if it exists, attaching the file
/// `name` afterwards.
#[inline]
fn file<P>(path: Option<PathBuf>, name: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    path.map(move |mut p| {
        p.push(PATH_IDENTIFIER);
        p.push(name);
        p
    })
}

/// Configuration
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Config {
    /// Root Seed File
    pub root_seed_file: PathBuf,

    /// Directory for Proving Keys
    pub proving_key_directory: PathBuf,

    /// Service URL
    pub service_url: String,

    /// Dev origin URL
    pub dev_origin_url: String,

    /// Prod origin URL
    pub prod_origin_url: String
}

impl Config {
    /// Tries to build a default [`Config`].
    #[inline]
    pub fn try_default() -> Option<Self> {
        Some(Self {
            root_seed_file: file(dirs_next::config_dir(), "root_seed.aes")?,
            proving_key_directory: directory(dirs_next::data_local_dir())?,
            service_url: String::from("http://127.0.0.1:29987"),
            dev_origin_url: String::from("http://localhost:8000"),
            prod_origin_url: String::from("https://dapp-alpha.manta.network")
        })
    }

    /// Runs the setup for the configuration directories. This step ensures that all of the
    /// directories exist for the current state of the configuration.
    #[inline]
    pub async fn setup(&self) -> io::Result<()> {
        // FIXME: Use logging instead of `println!`.
        // FIXME: Not using asynchronous `println!` because it interferes with `Send` requirements.
        println!("Manta Signer {:#?}\n", self);
        if let Some(parent) = self.root_seed_file.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::create_dir_all(&self.proving_key_directory).await?;
        Ok(())
    }

    /// Returns the path to a file in the [`self.proving_key_directory`].
    ///
    /// [`self.proving_key_directory`](Self::proving_key_directory)
    #[inline]
    pub fn proving_key_path<P>(&self, path: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        let mut directory = self.proving_key_directory.clone();
        directory.push(path);
        directory
    }

    /// Returns the path to the `PrivateTransfer` proving key.
    #[inline]
    pub fn private_transfer_proving_key_path(&self) -> PathBuf {
        self.proving_key_path("transfer_pk.bin")
    }

    /// Returns the path to the `Reclaim` proving key.
    #[inline]
    pub fn reclaim_proving_key_path(&self) -> PathBuf {
        self.proving_key_path("reclaim_pk.bin")
    }
}
