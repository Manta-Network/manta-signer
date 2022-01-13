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

//! Transaction Batching

use ark_serialize::CanonicalDeserialize;
use async_std::{fs, path::Path, task};
use bip32::XPrv;
use core::convert::TryInto;
use manta_api::{
    DeriveShieldedAddressParams, GenerateAssetParams, GeneratePrivateTransferBatchParams,
    GeneratePrivateTransferParams, GenerateReclaimBatchParams, GenerateReclaimParams,
    MantaRootSeed, PrivateTransferBatch, ReclaimBatch,
};
use manta_asset::{AssetId, MantaAsset, MantaAssetShieldedAddress, Process};
use manta_crypto::{commitment_parameters, leaf_parameters, two_to_one_parameters, Groth16Pk};
use manta_data::{BuildMetadata, PrivateTransferData, ReclaimData};
use rand::{CryptoRng, RngCore};

/// Loads proving key from `path`.
#[inline]
pub async fn load_proving_key<P>(path: P) -> Groth16Pk
where
    P: AsRef<Path>,
{
    Groth16Pk::deserialize_unchecked(
        fs::read(path)
            .await
            .expect("Failed to read file.")
            .as_slice(),
    )
    .expect("Failed to deserialize proving key.")
}

/// Generates batched private transfer data.
#[inline]
pub async fn batch_generate_private_transfer_data<P, R, F>(
    params: GeneratePrivateTransferBatchParams,
    root_seed: MantaRootSeed,
    private_transfer_pk_path: P,
    rng_source: F,
) -> PrivateTransferBatch
where
    P: AsRef<Path>,
    R: CryptoRng + RngCore,
    F: 'static + Copy + Send + Sync + Fn() -> R,
{
    let private_transfer_pk = load_proving_key(private_transfer_pk_path).await;
    let asset_id = params.asset_id;
    let receiving_address = params.receiving_address;
    let last_private_transfer_index = params.private_transfer_params_list.len() - 1;
    PrivateTransferBatch {
        private_transfer_data_list: futures::future::join_all(
            params
                .private_transfer_params_list
                .into_iter()
                .enumerate()
                .map(|(i, p)| {
                    let receiving_address = if i == last_private_transfer_index {
                        Some(receiving_address)
                    } else {
                        None
                    };
                    let private_transfer_pk = private_transfer_pk.clone();
                    task::spawn_blocking(move || {
                        generate_private_transfer_data(
                            p,
                            asset_id,
                            receiving_address,
                            root_seed,
                            private_transfer_pk,
                            &mut rng_source(),
                        )
                    })
                }),
        )
        .await,
    }
}

/// Generates batched reclaim data.
#[inline]
pub async fn batch_generate_reclaim_data<P, R, F>(
    params: GenerateReclaimBatchParams,
    root_seed: MantaRootSeed,
    private_transfer_pk_path: P,
    reclaim_pk_path: P,
    rng_source: F,
) -> ReclaimBatch
where
    P: AsRef<Path>,
    R: CryptoRng + RngCore,
    F: 'static + Copy + Send + Sync + Fn() -> R,
{
    // FIXME: Have the reclaim proof happen at the same time as the transfer proofs.
    let private_transfer_pk = load_proving_key(private_transfer_pk_path).await;
    let reclaim_pk = load_proving_key(reclaim_pk_path).await;
    let reclaim_params = params.reclaim_params;
    let asset_id = reclaim_params.asset_id;
    ReclaimBatch {
        reclaim_data: task::spawn_blocking(move || {
            generate_reclaim_data(
                reclaim_params,
                asset_id,
                root_seed,
                reclaim_pk,
                &mut rng_source(),
            )
        })
        .await,
        private_transfer_data_list: futures::future::join_all(
            params.private_transfer_params_list.into_iter().map(|p| {
                let private_transfer_pk = private_transfer_pk.clone();
                task::spawn_blocking(move || {
                    generate_private_transfer_data(
                        p,
                        asset_id,
                        None,
                        root_seed,
                        private_transfer_pk,
                        &mut rng_source(),
                    )
                })
            }),
        )
        .await,
    }
}

/// Generates one round of private transfer data.
#[inline]
pub fn generate_private_transfer_data<R>(
    params: GeneratePrivateTransferParams,
    asset_id: AssetId,
    receiving_address: Option<MantaAssetShieldedAddress>,
    root_seed: MantaRootSeed,
    transfer_pk: Groth16Pk,
    rng: &mut R,
) -> PrivateTransferData
where
    R: CryptoRng + RngCore,
{
    let commit_params = commitment_parameters();
    let leaf_params = leaf_parameters();
    let two_to_one_params = two_to_one_parameters();
    let sender_asset_1 = generate_asset(
        GenerateAssetParams {
            asset_id,
            value: params.sender_asset_1_value,
            keypath: params.sender_asset_1_keypath,
        },
        &root_seed,
    );
    let sender_asset_2 = generate_asset(
        GenerateAssetParams {
            asset_id,
            value: params.sender_asset_2_value,
            keypath: params.sender_asset_2_keypath,
        },
        &root_seed,
    );
    let sender_metadata_1 = sender_asset_1
        .build(
            &leaf_params,
            &two_to_one_params,
            &params.sender_asset_1_shard,
        )
        .expect("Failed to build sender 1 metadata");
    let sender_metadata_2 = sender_asset_2
        .build(
            &leaf_params,
            &two_to_one_params,
            &params.sender_asset_2_shard,
        )
        .expect("Failed to build sender 2 metadata");
    let non_change_receiving_address = match receiving_address {
        Some(receiving_address) => receiving_address,
        None => manta_api::derive_shielded_address(
            DeriveShieldedAddressParams {
                keypath: params.non_change_output_keypath.unwrap(),
            },
            &root_seed,
        ),
    };
    let non_change_processed_receiver = non_change_receiving_address
        .process(asset_id, params.non_change_output_value, rng)
        .expect("Failed to process receiver 1 shielded address");
    let change_address = manta_api::derive_shielded_address(
        DeriveShieldedAddressParams {
            keypath: params.change_output_keypath,
        },
        &root_seed,
    );
    let change_processed_receiver = change_address
        .process(asset_id, params.change_output_value, rng)
        .expect("Failed to process receiver 2 shielded address");
    manta_api::generate_private_transfer_struct(
        commit_params,
        leaf_params,
        two_to_one_params,
        &transfer_pk,
        [sender_metadata_1, sender_metadata_2],
        [non_change_processed_receiver, change_processed_receiver],
        rng,
    )
    .expect("Failed to generate private transfer payload from deserialized data")
}

/// Generates one round of reclaim data.
#[inline]
pub fn generate_reclaim_data<R>(
    params: GenerateReclaimParams,
    asset_id: AssetId,
    root_seed: MantaRootSeed,
    reclaim_pk: Groth16Pk,
    rng: &mut R,
) -> ReclaimData
where
    R: CryptoRng + RngCore,
{
    let commit_params = commitment_parameters();
    let leaf_params = leaf_parameters();
    let two_to_one_params = two_to_one_parameters();
    let input_asset_1 = generate_asset(
        GenerateAssetParams {
            asset_id,
            value: params.input_asset_1_value,
            keypath: params.input_asset_1_keypath,
        },
        &root_seed,
    );
    let input_asset_2 = generate_asset(
        GenerateAssetParams {
            asset_id,
            value: params.input_asset_2_value,
            keypath: params.input_asset_2_keypath,
        },
        &root_seed,
    );
    let sender_metadata_1 = input_asset_1
        .build(
            &leaf_params,
            &two_to_one_params,
            &params.input_asset_1_shard,
        )
        .expect("Failed to build sender 1 metadata");
    let sender_metadata_2 = input_asset_2
        .build(
            &leaf_params,
            &two_to_one_params,
            &params.input_asset_2_shard,
        )
        .expect("Failed to build sender 2 metadata");

    let change_address = manta_api::derive_shielded_address(
        DeriveShieldedAddressParams {
            keypath: params.change_keypath,
        },
        &root_seed,
    );
    let change_value =
        params.input_asset_1_value + params.input_asset_2_value - params.reclaim_value;
    let change_processed_receiver = change_address
        .process(asset_id, change_value, rng)
        .expect("Failed to build processed receiver");
    manta_api::generate_reclaim_struct(
        commit_params,
        leaf_params,
        two_to_one_params,
        &reclaim_pk,
        [sender_metadata_1, sender_metadata_2],
        change_processed_receiver,
        params.reclaim_value,
        rng,
    )
    .expect("Failed to generate reclaim struct")
}

/// Generates an asset from `params` and `root_seed`.
#[inline]
pub fn generate_asset(params: GenerateAssetParams, root_seed: &MantaRootSeed) -> MantaAsset {
    let asset_secret_key = XPrv::derive_from_path(
        root_seed.as_ref(),
        &params.keypath.parse().expect("Invalid derivation path"),
    )
    .expect("Failed to derive extended private key from path")
    .private_key()
    .to_bytes()
    .try_into()
    .unwrap();
    MantaAsset::new(
        asset_secret_key,
        &commitment_parameters(),
        params.asset_id,
        params.value,
    )
    .expect("Failed to sample asset")
}
