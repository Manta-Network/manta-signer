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

//! Manta Signer

#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![forbid(rustdoc::broken_intra_doc_links)]
#![forbid(missing_docs)]

extern crate alloc;

#[cfg(feature = "config")]
pub mod config;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "log")]
pub mod log;
#[cfg(feature = "network")]
pub mod network;
#[cfg(feature = "parameters")]
pub mod parameters;
#[cfg(feature = "secret")]
pub mod secret;
#[cfg(feature = "service")]
pub mod service;
#[cfg(feature = "storage")]
pub mod storage;

#[doc(inline)]
pub use manta_util::serde;

#[doc(inline)]
pub use tokio;

/// Manta Signer Server Version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
