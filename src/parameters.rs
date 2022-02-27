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

use manta_pay::{
    config::{
        KeyAgreementScheme, MultiProvingContext, Parameters, ProvingContext, UtxoCommitmentScheme,
        UtxoSetModel, VoidNumberHashFunction,
    },
    signer::base::SignerParameters,
};
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
    let mint = directory.join("mint.dat");
    manta_sdk::pay::testnet::proving::Mint::download_if_invalid(&mint).ok()?;
    let private_transfer = directory.join("private-transfer.dat");
    manta_sdk::pay::testnet::proving::PrivateTransfer::download_if_invalid(&private_transfer)
        .ok()?;
    let reclaim = directory.join("reclaim.dat");
    manta_sdk::pay::testnet::proving::Reclaim::download_if_invalid(&reclaim).ok()?;
    Some(SignerParameters {
        proving_context: MultiProvingContext {
            mint: ProvingContext::decode(IoReader(File::open(mint).ok()?)).ok()?,
            private_transfer: ProvingContext::decode(IoReader(File::open(private_transfer).ok()?))
                .ok()?,
            reclaim: ProvingContext::decode(IoReader(File::open(reclaim).ok()?)).ok()?,
        },
        parameters: Parameters {
            key_agreement: KeyAgreementScheme::decode(
                manta_sdk::pay::testnet::parameters::KeyAgreement::get()?,
            )
            .ok()?,
            utxo_commitment: UtxoCommitmentScheme::decode(
                manta_sdk::pay::testnet::parameters::UtxoCommitmentScheme::get()?,
            )
            .ok()?,
            void_number_hash: VoidNumberHashFunction::decode(
                manta_sdk::pay::testnet::parameters::VoidNumberHashFunction::get()?,
            )
            .ok()?,
        },
    })
}

/// Loads the [`UtxoSetModel`] from the Manta SDK.
#[inline]
pub fn load_utxo_set_model() -> Option<UtxoSetModel> {
    UtxoSetModel::decode(manta_sdk::pay::testnet::parameters::UtxoSetParameters::get()?).ok()
}
