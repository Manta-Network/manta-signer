extern crate libc;

use std::ffi::{CStr, CString};
use std::convert::TryInto;

use manta_api::derive_shielded_address as _derive_shielded_address;
use manta_api::generate_ui_asset as _generate_ui_asset;
use manta_api::generate_private_transfer_data as _generate_private_transfer_data;
use manta_api::generate_mint_data as _generate_mint_data;
use manta_api::generate_reclaim_data as _generate_reclaim_data;
use manta_api::recover_account as _recover_account;
use manta_api::load_root_seed as _load_root_seed;
use manta_api::save_root_seed;

use manta_api::{
    DeriveShieldedAddressParams, GenerateAssetParams, GeneratePrivateTransferDataParams,
    GenerateReclaimDataParams, RecoverAccountParams,
};

use bip0039::{Mnemonic, Count};
use codec::Decode;
use codec::Encode;
use manta_asset::MantaSecretKey;
use manta_crypto::MantaSerDes;
use rand::thread_rng;
use rand::RngCore;
use rand_chacha::ChaCha20Rng;
use rand_chacha::rand_core::SeedableRng;


#[no_mangle]
pub extern "C" fn load_root_seed(password: *const libc::c_char, out: *mut *mut u8) -> libc::size_t {
    let password: &CStr = unsafe { CStr::from_ptr(password) };
    let password: String = password.to_str().unwrap().to_owned();
    let result = _load_root_seed(password);
    match result {
        Ok(root_seed) => {
            let mut buf = root_seed.to_vec();
            let ptr = buf.as_mut_ptr();
            std::mem::forget(buf);
            unsafe {
                *out = ptr;
            }
            0
        }
        Err(_) => {
            1
        }
    }
}

// todo: modify this to return mnemonic
#[no_mangle]
pub extern "C" fn create_account(
    password: *const libc::c_char,
    out: *mut *mut libc::c_char,
    out_len: *mut libc::size_t,
) -> libc::size_t {
    let password: &CStr = unsafe { CStr::from_ptr(password) };
    let password: String = password.to_str().unwrap().to_owned();
    // todo: this is looking extremely not random
    let recovery_phrase = Mnemonic::generate(Count::Words12);
    let root_seed = recovery_phrase.to_seed("");

    save_root_seed(root_seed, password);

    let recovery_phrase_string = recovery_phrase.into_phrase();
    let len = recovery_phrase_string.len();
    let recovery_phrase_c_string = CString::new(recovery_phrase_string).expect("CString::new failed");
    unsafe {
        *out = recovery_phrase_c_string.into_raw();
        *out_len = len;
    }
    0
}

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
    let mut rng = get_crypto_rng();
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
    let mut rng = get_crypto_rng();
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

fn get_crypto_rng() -> ChaCha20Rng {
    let mut rng = thread_rng();
    let mut crypto_rng_seed = [0u8; 32];
    rng.fill_bytes(&mut crypto_rng_seed);

    ChaCha20Rng::from_seed(crypto_rng_seed)
}
