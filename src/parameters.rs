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
use std::fs::File;

/// Loads the [`SignerParameters`] from the Manta SDK.
#[inline]
pub fn load() -> Option<SignerParameters> {
    let directory = tempfile::tempdir().ok()?;
    let path = directory.path();
    let mint_path = path.join("mint.dat");
    manta_sdk::pay::testnet::proving::Mint::download(&mint_path).ok()?;
    let private_transfer_path = path.join("private-transfer.dat");
    manta_sdk::pay::testnet::proving::PrivateTransfer::download(&private_transfer_path).ok()?;
    let reclaim_path = path.join("reclaim.dat");
    manta_sdk::pay::testnet::proving::Reclaim::download(&reclaim_path).ok()?;
    let parameters = SignerParameters {
        proving_context: MultiProvingContext {
            mint: ProvingContext::decode(IoReader(File::open(mint_path).ok()?)).ok()?,
            private_transfer: ProvingContext::decode(IoReader(
                File::open(private_transfer_path).ok()?,
            ))
            .ok()?,
            reclaim: ProvingContext::decode(IoReader(File::open(reclaim_path).ok()?)).ok()?,
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
    };
    directory.close().ok()?;
    Some(parameters)
}

/// Loads the [`UtxoSetModel`] from the Manta SDK.
#[inline]
pub fn load_utxo_set_model() -> Option<UtxoSetModel> {
    UtxoSetModel::decode(manta_sdk::pay::testnet::parameters::UtxoSetParameters::get()?).ok()
}
