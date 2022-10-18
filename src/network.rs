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

//! Manta Signer Multi-Network Support

use core::{
    fmt::{self, Display},
    ops::{Index, IndexMut},
};
use manta_util::serde::{Deserialize, Serialize};

/// Network Type
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields, crate = "manta_util::serde")]
pub enum Network {
    /// Dolphin Testnet
    Dolphin,

    /// Calamari Network
    Calamari,

    /// Manta Network
    Manta,
}

impl Display for Network {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Network::Dolphin => "Dolphin".fmt(f),
            Network::Calamari => "Calamari".fmt(f),
            Network::Manta => "Manta".fmt(f),
        }
    }
}

/// Network-Specific Data
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields, crate = "manta_util::serde")]
pub struct NetworkSpecific<T> {
    /// Dolphin Data
    pub dolphin: T,

    /// Calamari Data
    pub calamari: T,

    /// Manta Data
    pub manta: T,
}

impl<T> Index<Network> for NetworkSpecific<T> {
    type Output = T;

    #[inline]
    fn index(&self, network: Network) -> &Self::Output {
        match network {
            Network::Dolphin => &self.dolphin,
            Network::Calamari => &self.calamari,
            Network::Manta => &self.manta,
        }
    }
}

impl<T> IndexMut<Network> for NetworkSpecific<T> {
    #[inline]
    fn index_mut(&mut self, network: Network) -> &mut Self::Output {
        match network {
            Network::Dolphin => &mut self.dolphin,
            Network::Calamari => &mut self.calamari,
            Network::Manta => &mut self.manta,
        }
    }
}

/// Network-Specific Message
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields, crate = "manta_util::serde")]
pub struct Message<T> {
    /// Network Type
    pub network: Network,

    /// Message Content
    pub message: T,
}
