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

//! Manta Signer SDK Parameter Loading

// TODO: Report a more informative error.

use manta_parameters::{Download, Get};
use manta_pay::{config, parameters::load_transfer_parameters, signer::base::SignerParameters};
use manta_util::codec::{Decode, IoReader};
use std::{
    fs::{self, File},
    path::Path,
};

/// Loads the [`SignerParameters`] from the Manta SDK.
#[inline]
pub fn load<P>(directory: P) -> Option<SignerParameters>
where
    P: AsRef<Path>,
{
    let mut directory = directory.as_ref().to_owned();
    directory.push("sdk");
    directory.push("data");
    directory.push("pay");
    directory.push("testnet");
    directory.push("proving");
    fs::create_dir_all(&directory).ok()?;
    let mint = directory.join("to-private.dat");
    manta_parameters::pay::testnet::proving::ToPrivate::download_if_invalid(&mint).ok()?;
    let private_transfer = directory.join("private-transfer.dat");
    manta_parameters::pay::testnet::proving::PrivateTransfer::download_if_invalid(
        &private_transfer,
    )
    .ok()?;
    let reclaim = directory.join("to-public.dat");
    manta_parameters::pay::testnet::proving::ToPublic::download_if_invalid(&reclaim).ok()?;
    let parameters = load_transfer_parameters();
    Some(SignerParameters {
        proving_context: config::MultiProvingContext {
            to_private: config::ProvingContext::decode(IoReader(File::open(mint).ok()?)).ok()?,
            private_transfer: config::ProvingContext::decode(IoReader(
                File::open(private_transfer).ok()?,
            ))
            .ok()?,
            to_public: config::ProvingContext::decode(IoReader(File::open(reclaim).ok()?)).ok()?,
        },
        parameters,
    })
}

/// Loads the [`UtxoAccumulatorModel`](config::UtxoAccumulatorModel) from the Manta SDK.
#[inline]
pub fn load_utxo_accumulator_model() -> Option<config::UtxoAccumulatorModel> {
    config::UtxoAccumulatorModel::decode(
        manta_parameters::pay::testnet::parameters::UtxoAccumulatorModel::get()?,
    )
    .ok()
}
