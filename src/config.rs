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

use manta_pay::{key::Mnemonic};
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
    /// Data File Path - Dolphin
    pub data_path_dolphin: PathBuf,

    /// Data File Path - Calamari
    pub data_path_calamari: PathBuf,

    /// Data File Path - Manta
    pub data_path_manta: PathBuf,

    /// Service URL
    pub service_url: String,

    /// Origin URL
    pub origin_url: Option<String>,
}

impl Config {
    /// Tries to build a default [`Config`].
    #[inline]
    pub fn try_default() -> Option<Self> {
        Some(Self {
            data_path_dolphin: file(dirs_next::config_dir(), "storage-dolphin.dat")?,
            data_path_calamari: file(dirs_next::config_dir(), "storage-calamari.dat")?,
            data_path_manta: file(dirs_next::config_dir(), "storage-manta.dat")?,
            service_url: "127.0.0.1:29987".into(),
            #[cfg(feature = "unsafe-disable-cors")]
            origin_url: None,
            #[cfg(not(feature = "unsafe-disable-cors"))]
            origin_url: Some("https://app.dolphin.manta.network".into()),
        })
    }

    /// Returns the data directory path. All files will be in same directory
    /// so it suffices to check on one file i.e. Dolphin.
    #[inline]
    pub fn data_directory(&self) -> &Path {
        self.data_path_dolphin
            .parent()
            .expect("The data path file must always have a parent.")
    }

    /// Returns whether or not storage file exists already on the filesystem resources.
    #[inline]
    pub async fn does_data_exist(&self) -> io::Result<bool> {
        fs::create_dir_all(self.data_directory()).await?;
        match fs::metadata(&self.data_path_dolphin).await {
            Ok(metadata) if metadata.is_file() => Ok(true),
            Ok(metadata) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Invalid file format: {:?}.", metadata)
            )),
            _ => Ok(false)
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
