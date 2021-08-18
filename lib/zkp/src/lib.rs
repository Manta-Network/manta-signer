extern crate libc;

use manta_api::signer::hd_wallet::derive_shielded_address as _derive_shielded_address;
use manta_api::signer::payload_gen::generate_ui_asset as _generate_ui_asset;
use manta_api::signer::payload_gen::generate_private_transfer_data as _generate_private_transfer_data;
use manta_api::signer::payload_gen::generate_mint_data as _generate_mint_data;
use manta_api::signer::payload_gen::generate_reclaim_data as _generate_reclaim_data;
use manta_api::signer::recover_account::recover_account as _recover_account;

use manta_api::signer::params::{
    DeriveShieldedAddressParams, GenerateAssetParams, GeneratePrivateTransferDataParams,
    GenerateReclaimDataParams, RecoverAccountParams
};

use codec::Decode;
use manta_asset::MantaSecretKey;
use manta_crypto::MantaSerDes;
use rand::thread_rng;
use codec::Encode;

#[no_mangle]
pub extern "C" fn derive_shielded_address(
    buffer: *const libc::c_uchar,
    len: libc::size_t,
    out: *mut *mut u8,
    out_len: *mut libc::size_t,
) -> libc::size_t {
    let mut bytes: &[u8] = unsafe { std::slice::from_raw_parts(buffer, len) };
    let params = DeriveShieldedAddressParams::decode(&mut bytes).unwrap();
    let root_seed: MantaSecretKey = [0u8; 32].into();
    let shielded_address = _derive_shielded_address(params, &root_seed);
    let mut buf: Vec<u8> = vec![];
    shielded_address.serialize(&mut buf).unwrap();
    let len = buf.len();
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    unsafe {
        *out = ptr;
        *out_len = len;
    }
    0
}

#[no_mangle]
pub extern "C" fn generate_asset(
    buffer: *const libc::c_uchar,
    len: libc::size_t,
    out: *mut *mut u8,
    out_len: *mut libc::size_t,
) -> libc::size_t {
    let mut bytes: &[u8] = unsafe { std::slice::from_raw_parts(buffer, len) };
    let params = GenerateAssetParams::decode(&mut bytes).unwrap();
    let root_seed: MantaSecretKey = [0u8; 32].into();
    let asset = _generate_ui_asset(params, &root_seed);
    let mut buf = asset.encode();
    let len = buf.len();
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    unsafe {
        *out = ptr;
        *out_len = len;
    }
    0
}

#[no_mangle]
pub extern "C" fn generate_mint_data(
    buffer: *const libc::c_uchar,
    len: libc::size_t,
    out: *mut *mut u8,
    out_len: *mut libc::size_t,
) -> libc::size_t {
    let mut bytes: &[u8] = unsafe { std::slice::from_raw_parts(buffer, len) };
    let params = GenerateAssetParams::decode(&mut bytes).unwrap();
    let root_seed: MantaSecretKey = [0u8; 32].into();
    let mint_data = _generate_mint_data(params, &root_seed);
    let mut buf: Vec<u8> = vec![];
    mint_data.serialize(&mut buf).unwrap();
    let len = buf.len();
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    unsafe {
        *out = ptr;
        *out_len = len;
    }
    0
}

#[no_mangle]
pub extern "C" fn generate_private_transfer_data(
    buffer: *const libc::c_uchar,
    len: libc::size_t,
    out: *mut *mut u8,
    out_len: *mut libc::size_t,
) -> libc::size_t {
    let mut bytes: &[u8] = unsafe { std::slice::from_raw_parts(buffer, len) };
    let params = GeneratePrivateTransferDataParams::decode(&mut bytes).unwrap();
    let root_seed: MantaSecretKey = [0u8; 32].into();
    let proving_key_path = "./lib/zkp/keys/transfer_pk.bin";
    let mut rng = thread_rng();
    let private_transfer_data =
        _generate_private_transfer_data(params, &root_seed, &proving_key_path, &mut rng);
    let mut buf: Vec<u8> = vec![];
    private_transfer_data.serialize(&mut buf).unwrap();
    let len = buf.len();
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    unsafe {
        *out = ptr;
        *out_len = len;
    }
    0
}

#[no_mangle]
pub extern "C" fn generate_reclaim_data(
    buffer: *const libc::c_uchar,
    len: libc::size_t,
    out: *mut *mut u8,
    out_len: *mut libc::size_t,
) -> libc::size_t {
    let mut bytes: &[u8] = unsafe { std::slice::from_raw_parts(buffer, len) };
    let params = GenerateReclaimDataParams::decode(&mut bytes).unwrap();
    let root_seed: MantaSecretKey = [0u8; 32].into();
    let proving_key_path = "./lib/zkp/keys/reclaim_pk.bin";
    let mut rng = thread_rng();
    let reclaim_data = _generate_reclaim_data(params, &root_seed, &proving_key_path, &mut rng);
    let mut buf: Vec<u8> = vec![];
    reclaim_data.serialize(&mut buf).unwrap();
    let len = buf.len();
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    unsafe {
        *out = ptr;
        *out_len = len;
    }
    0
}

#[no_mangle]
pub extern "C" fn recover_account(
    buffer: *const libc::c_uchar,
    len: libc::size_t,
    out: *mut *mut u8,
    out_len: *mut libc::size_t,
) -> libc::size_t {
    let mut bytes: &[u8] = unsafe { std::slice::from_raw_parts(buffer, len) };
    let params = RecoverAccountParams::decode(&mut bytes).unwrap();
    let root_seed: MantaSecretKey = [0u8; 32].into();
    let account = _recover_account(params, &root_seed);
    let mut buf = account.encode();
    let len = buf.len();
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    unsafe {
        *out = ptr;
        *out_len = len;
    }
    0
}
