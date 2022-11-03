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

//! Manta Signer Configuration

use manta_crypto::rand::OsRng;
use manta_pay::key::Mnemonic;
use manta_util::serde::{Deserialize, Serialize};
use std::{
    io,
    path::{Path, PathBuf},
};
use tokio::fs;

/// Manta Path Identifier
pub const PATH_IDENTIFIER: &str = "manta-signer";

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
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(crate = "manta_util::serde", deny_unknown_fields)]
pub struct Config {
    /// Data File Path
    pub data_path: PathBuf,

    /// Backup File Path
    pub backup_data_path: PathBuf,

    /// Service URL
    ///
    /// This URL defines the listening URL for the service.
    pub service_url: String,

    /// Origin URLs
    ///
    /// These URLs are the allowed origins that can send requests to the service. An empty list
    /// means any origin is allowed to send requests to the service.
    pub origin_urls: Vec<String>,
}

impl Config {
    /// Tries to build a default [`Config`].
    #[inline]
    pub fn try_default() -> Option<Self> {
        Some(Self {
            data_path: file(dirs_next::config_dir(), "storage.dat")?,
            backup_data_path: file(dirs_next::config_dir(), "storage.backup")?,
            service_url: "127.0.0.1:29987".into(),
            #[cfg(feature = "unsafe-disable-cors")]
            origin_urls: vec![],
            #[cfg(not(feature = "unsafe-disable-cors"))]
            origin_urls: vec![
                "https://app.manta.network".into(),
                "https://app.dolphin.manta.network".into(),
            ],
        })
    }

    /// Returns the data directory path.
    #[inline]
    pub fn data_directory(&self) -> &Path {
        self.data_path
            .parent()
            .expect("The data path file must always have a parent.")
    }

    /// Builds the [`Setup`] for the given configuration depending on the filesystem resources.
    #[inline]
    pub async fn setup(&self) -> io::Result<Setup> {
        fs::create_dir_all(self.data_directory()).await?;
        match fs::metadata(&self.data_path).await {
            Ok(metadata) if metadata.is_file() => Ok(Setup::Login),
            Ok(metadata) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Invalid file format: {:?}.", metadata),
            )),
            _ => Ok(Setup::CreateAccount(Mnemonic::sample(&mut OsRng))),
        }
    }

    /// Checks for existence of backup storage file. If found, will set the backup
    /// to be the default storage file.
    #[inline]
    pub async fn check_for_backup(&self) -> io::Result<bool> {
        fs::create_dir_all(self.data_directory()).await?;
        match fs::metadata(&self.backup_data_path).await {
            Ok(metadata) if metadata.is_file() => {
                // need to check if old storage file still exists before deleting.
                if let Ok(metadata) = fs::metadata(&self.data_path).await {
                    if metadata.is_file() {
                        fs::remove_file(self.data_path.clone()).await?;
                    }
                }
                
                fs::rename(self.backup_data_path.clone(), self.data_path.clone()).await?;
                Ok(true)
            }
            Ok(metadata) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Invalid file format: {:?}.", metadata),
            )),
            _ => Ok(false),
        }
    }
}

/// Setup Phase
#[derive(Clone, Deserialize, Serialize)]
#[serde(
    content = "content",
    crate = "manta_util::serde",
    deny_unknown_fields,
    tag = "type"
)]
pub enum Setup {
    /// Create Account
    CreateAccount(Mnemonic),

    /// Login
    Login,
}
