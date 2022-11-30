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

use manta_pay::{
    key::Mnemonic,
    signer::client::network::{Network, NetworkSpecific},
};
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
    pub data_path: NetworkSpecific<PathBuf>,

    /// Backup File Path
    pub backup_data_path: NetworkSpecific<PathBuf>,

    /// Service URL
    ///
    /// This URL defines the listening URL for the service.
    pub service_url: String,

    /// Origin URLs
    ///
    /// These URLs are the allowed origins that can send requests to the service. An empty list
    /// means any origin is allowed to send requests to the service.
    pub origin_urls: Vec<String>,

    /// If the Signer App can perform a full restart. This is required in order to properly
    /// terminate the running http sever and disconnect from the UI, but only works in
    /// normal builds, not dev mode.
    /// Thus when running `cargo dev` the feature `disable-restart` must be enabled.
    pub can_app_restart: bool,
}

/// Response for the [`Config::does_data_exist`] function of [`Config`]. The boolean fields
/// represent whether or not each respective network data file exists or not.
pub type DataExistenceResponse = NetworkSpecific<bool>;

impl Config {
    /// Tries to build a default [`Config`].
    #[inline]
    pub fn try_default() -> Option<Self> {
        Some(Self {
            data_path: NetworkSpecific {
                dolphin: file(dirs_next::config_dir(), "storage-dolphin.dat")?,
                calamari: file(dirs_next::config_dir(), "storage-calamari.dat")?,
                manta: file(dirs_next::config_dir(), "storage-manta.dat")?,
            },
            backup_data_path: NetworkSpecific {
                dolphin: file(dirs_next::config_dir(), "storage-dolphin.backup")?,
                calamari: file(dirs_next::config_dir(), "storage-calamari.backup")?,
                manta: file(dirs_next::config_dir(), "storage-manta.backup")?,
            },
            service_url: "127.0.0.1:29987".into(),
            #[cfg(feature = "unsafe-disable-cors")]
            origin_urls: vec![],
            #[cfg(not(feature = "unsafe-disable-cors"))]
            origin_urls: vec![
                "https://app.manta.network".into(),
                "https://app.dolphin.manta.network".into(),
            ],
            #[cfg(feature = "disable-restart")]
            can_app_restart: false,
            #[cfg(not(feature = "disable-restart"))]
            can_app_restart: true,
        })
    }

    /// Returns the data directory path. All files will be in same directory so it suffices to check
    /// on one file i.e. Dolphin.
    #[inline]
    pub fn data_directory(&self) -> &Path {
        self.data_path
            .dolphin
            .parent()
            .expect("The data path file must always have a parent.")
    }

    /// Returns whether storage file exists for a particular data path.
    #[inline]
    async fn does_data_exist_at(path: &PathBuf) -> io::Result<bool> {
        match fs::metadata(path).await {
            Ok(metadata) if metadata.is_file() => Ok(true),
            Ok(metadata) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Invalid file format: {:?}.", metadata),
            )),
            _ => Ok(false),
        }
    }

    /// Returns whether or not storage files exist already on the filesystem resources.
    #[inline]
    pub async fn does_data_exist(&self) -> DataExistenceResponse {
        let _ = fs::create_dir_all(self.data_directory()).await;

        let dolphin_exists = Self::does_data_exist_at(&self.data_path.dolphin)
            .await
            .expect("Unable to read dolphin file.");
        let calamari_exists = Self::does_data_exist_at(&self.data_path.calamari)
            .await
            .expect("Unable to read calamari file.");
        let manta_exists = Self::does_data_exist_at(&self.data_path.manta)
            .await
            .expect("Unable to read manta file.");

        DataExistenceResponse {
            dolphin: dolphin_exists,
            calamari: calamari_exists,
            manta: manta_exists,
        }
    }

    /// Checks for existence of backup storage file for a particular network.
    /// If found, will set the backup to be the default storage file.
    #[inline]
    pub async fn check_for_backup(&self, network: Network) -> io::Result<bool> {
        fs::create_dir_all(self.data_directory()).await?;
        match fs::metadata(&self.backup_data_path[network]).await {
            Ok(metadata) if metadata.is_file() => {
                // need to check if old storage file still exists before deleting.
                if let Ok(metadata) = fs::metadata(&self.data_path[network]).await {
                    if metadata.is_file() {
                        fs::remove_file(self.data_path[network].clone()).await?;
                    }
                }
                fs::rename(
                    self.backup_data_path[network].clone(),
                    self.data_path[network].clone(),
                )
                .await?;
                Ok(true)
            }
            Ok(metadata) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Invalid file format: {:?}.", metadata),
            )),
            _ => Ok(false),
        }
    }

    /// Checks if backup exists for all networks, if it does then restores
    /// using the backup.
    #[inline]
    pub async fn check_all_backups(&self) -> io::Result<bool> {
        let dolphin_backup_exists = self
            .check_for_backup(Network::Dolphin)
            .await
            .expect("unable to check for Dolphin backup");
        let calamari_backup_exists = self
            .check_for_backup(Network::Calamari)
            .await
            .expect("unable to check for Calamari backup");
        let manta_backup_exists = self
            .check_for_backup(Network::Manta)
            .await
            .expect("unable to check for Manta backup");

        Ok(dolphin_backup_exists && calamari_backup_exists && manta_backup_exists)
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
