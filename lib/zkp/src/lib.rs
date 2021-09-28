extern crate libc;

use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::str::FromStr;

use manta_api::batch_generate_private_transfer_data as _batch_generate_private_transfer_data;
use manta_api::batch_generate_reclaim_data as _batch_generate_reclaim_data;
use manta_api::derive_shielded_address as _derive_shielded_address;
use manta_api::generate_mint_data as _generate_mint_data;
use manta_api::generate_signer_input_asset as _generate_signer_input_asset;
use manta_api::load_root_seed as _load_root_seed;
use manta_api::recover_account as _recover_account;

use manta_api::get_private_transfer_batch_params_recipient as _get_private_transfer_batch_params_recipient;
use manta_api::get_private_transfer_batch_params_currency_symbol as _get_private_transfer_batch_params_currency_symbol;
use manta_api::get_private_transfer_batch_params_value as _get_private_transfer_batch_params_value;
use manta_api::get_reclaim_batch_params_value as _get_reclaim_batch_params_value;
use manta_api::get_reclaim_batch_params_currency_symbol as _get_reclaim_batch_params_currency_symbol;

use manta_api::save_root_seed;
use manta_api::{GeneratePrivateTransferBatchParams, GenerateReclaimBatchParams};

use manta_api::{
    DeriveShieldedAddressParams, GenerateAssetParams, MantaRootSeed, RecoverAccountParams,
};

use bip0039::{Count, Mnemonic};
use codec::Decode;
use codec::Encode;
use manta_crypto::MantaSerDes;
use rand::thread_rng;
use rand::RngCore;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;

const OKAY: libc::size_t = 0;
const LOAD_ROOT_SEED_ERROR: libc::size_t = 1;
const BAD_PARAMETERS_ERROR: libc::size_t = 2;
const DESERIALIZE_ROOT_SEED_ERROR: libc::size_t = 3;
const SAVE_ROOT_SEED_ERROR: libc::size_t = 4;
const BAD_RECOVERY_PHRASE_ERROR: libc::size_t = 5;
const INVALID_ASSET_ID_ERROR: libc::size_t = 6;

#[no_mangle]
pub unsafe extern "C" fn load_root_seed(
    password: *const libc::c_char,
    out: *mut *mut u8,
) -> libc::size_t {
    let password: &CStr = CStr::from_ptr(password);

    let password: String = password.to_str().unwrap().to_owned();
    let result = _load_root_seed(password);
    match result {
        Ok(root_seed) => {
            let mut buf = root_seed.to_vec();
            let ptr = buf.as_mut_ptr();
            std::mem::forget(buf);
            *out = ptr;

            OKAY
        }
        Err(_) => LOAD_ROOT_SEED_ERROR,
    }
}

#[no_mangle]
pub unsafe extern "C" fn save_recovered_account(
    password: *const libc::c_char,
    recovery_phrase: *const libc::c_char,
) -> libc::size_t {
    let password: &CStr = CStr::from_ptr(password);
    let password: String = password.to_str().unwrap().to_owned();
    let recovery_phrase: &CStr = CStr::from_ptr(recovery_phrase);
    let recovery_phrase: String = recovery_phrase.to_str().unwrap().to_owned();
    let recovery_phrase: Mnemonic = match Mnemonic::from_str(&recovery_phrase) {
        Ok(mnemonic) => mnemonic,
        Err(_) => return BAD_RECOVERY_PHRASE_ERROR,
    };
    let root_seed = recovery_phrase.to_seed("");
    if save_root_seed(root_seed, password).is_err() {
        return SAVE_ROOT_SEED_ERROR;
    };
    OKAY
}

#[no_mangle]
pub unsafe extern "C" fn create_account(
    password: *const libc::c_char,
    out: *mut *mut libc::c_char,
) -> libc::size_t {
    let password: &CStr = CStr::from_ptr(password);
    let password: String = password.to_str().unwrap().to_owned();
    let recovery_phrase = Mnemonic::generate(Count::Words12);
    let root_seed = recovery_phrase.to_seed("");

    if save_root_seed(root_seed, password).is_err() {
        return SAVE_ROOT_SEED_ERROR;
    };

    let recovery_phrase_string = recovery_phrase.into_phrase();
    let recovery_phrase_c_string =
        CString::new(recovery_phrase_string).expect("CString::new failed");

    *out = recovery_phrase_c_string.into_raw();

    OKAY
}

#[no_mangle]
pub unsafe extern "C" fn derive_shielded_address(
    root_seed: *const libc::c_uchar,
    buffer: *const libc::c_uchar,
    len: libc::size_t,
    out: *mut *mut u8,
    out_len: *mut libc::size_t,
) -> libc::size_t {
    let root_seed: MantaRootSeed = match deserialize_root_seed(root_seed) {
        Ok(root_seed) => root_seed,
        Err(_) => return DESERIALIZE_ROOT_SEED_ERROR,
    };
    let mut bytes: &[u8] = std::slice::from_raw_parts(buffer, len);
    let params = match DeriveShieldedAddressParams::decode(&mut bytes) {
        Ok(params) => params,
        Err(_) => return BAD_PARAMETERS_ERROR,
    };
    let shielded_address = _derive_shielded_address(params, &root_seed);
    let mut buf: Vec<u8> = vec![];
    shielded_address.serialize(&mut buf).unwrap();
    let len = buf.len();
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);

    *out = ptr;
    *out_len = len;

    OKAY
}

#[no_mangle]
pub unsafe extern "C" fn generate_asset(
    root_seed: *const libc::c_uchar,
    buffer: *const libc::c_uchar,
    len: libc::size_t,
    out: *mut *mut u8,
    out_len: *mut libc::size_t,
) -> libc::size_t {
    let root_seed: MantaRootSeed = match deserialize_root_seed(root_seed) {
        Ok(root_seed) => root_seed,
        Err(_) => return DESERIALIZE_ROOT_SEED_ERROR,
    };
    let mut bytes: &[u8] = std::slice::from_raw_parts(buffer, len);
    let params = match GenerateAssetParams::decode(&mut bytes) {
        Ok(params) => params,
        Err(_) => return BAD_PARAMETERS_ERROR,
    };
    let asset = _generate_signer_input_asset(params, &root_seed);
    let mut buf = asset.encode();
    let len = buf.len();
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);

    *out = ptr;
    *out_len = len;

    OKAY
}

#[no_mangle]
pub unsafe extern "C" fn generate_mint_data(
    root_seed: *const libc::c_uchar,
    buffer: *const libc::c_uchar,
    len: libc::size_t,
    out: *mut *mut u8,
    out_len: *mut libc::size_t,
) -> libc::size_t {
    let root_seed: MantaRootSeed = match deserialize_root_seed(root_seed) {
        Ok(root_seed) => root_seed,
        Err(_) => return DESERIALIZE_ROOT_SEED_ERROR,
    };
    let mut bytes: &[u8] = std::slice::from_raw_parts(buffer, len);
    let params = match GenerateAssetParams::decode(&mut bytes) {
        Ok(params) => params,
        Err(_) => return BAD_PARAMETERS_ERROR,
    };
    let mint_data = _generate_mint_data(params, &root_seed);
    let mut buf: Vec<u8> = vec![];
    mint_data.serialize(&mut buf).unwrap();
    let len = buf.len();
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);

    *out = ptr;
    *out_len = len;

    OKAY
}

#[no_mangle]
pub unsafe extern "C" fn batch_generate_private_transfer_data(
    root_seed: *const libc::c_uchar,
    buffer: *const libc::c_uchar,
    len: libc::size_t,
    out: *mut *mut u8,
    out_len: *mut libc::size_t,
) -> libc::size_t {
    let root_seed: MantaRootSeed = match deserialize_root_seed(root_seed) {
        Ok(root_seed) => root_seed,
        Err(_) => return DESERIALIZE_ROOT_SEED_ERROR,
    };
    let mut bytes: &[u8] = std::slice::from_raw_parts(buffer, len);
    let params_batch = match GeneratePrivateTransferBatchParams::decode(&mut bytes) {
        Ok(params_batch) => params_batch,
        Err(_) => return BAD_PARAMETERS_ERROR,
    };
    let proving_key_path = "./lib/zkp/keys/transfer_pk.bin";
    let mut rng = get_crypto_rng();
    let private_transfer_data_batch = _batch_generate_private_transfer_data(
        params_batch,
        &root_seed,
        &proving_key_path,
        &mut rng,
    );
    let mut buf = private_transfer_data_batch.encode();
    let len = buf.len();
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);

    *out = ptr;
    *out_len = len;

    OKAY
}

#[no_mangle]
pub unsafe extern "C" fn batch_generate_reclaim_data(
    root_seed: *const libc::c_uchar,
    buffer: *const libc::c_uchar,
    len: libc::size_t,
    out: *mut *mut u8,
    out_len: *mut libc::size_t,
) -> libc::size_t {
    let root_seed: MantaRootSeed = match deserialize_root_seed(root_seed) {
        Ok(root_seed) => root_seed,
        Err(_) => return DESERIALIZE_ROOT_SEED_ERROR,
    };
    let mut bytes: &[u8] = std::slice::from_raw_parts(buffer, len);
    let params_batch = match GenerateReclaimBatchParams::decode(&mut bytes) {
        Ok(params_batch) => params_batch,
        Err(_) => return BAD_PARAMETERS_ERROR,
    };
    let proving_key_path = "./lib/zkp/keys/reclaim_pk.bin";
    let mut rng = get_crypto_rng();
    let reclaim_data_batch =
        _batch_generate_reclaim_data(params_batch, &root_seed, &proving_key_path, &mut rng);
    let mut buf = reclaim_data_batch.encode();
    let len = buf.len();
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);

    *out = ptr;
    *out_len = len;

    OKAY
}

#[no_mangle]
pub unsafe extern "C" fn recover_account(
    root_seed: *const libc::c_uchar,
    buffer: *const libc::c_uchar,
    len: libc::size_t,
    out: *mut *mut u8,
    out_len: *mut libc::size_t,
) -> libc::size_t {
    let root_seed: MantaRootSeed = match deserialize_root_seed(root_seed) {
        Ok(root_seed) => root_seed,
        Err(_) => return DESERIALIZE_ROOT_SEED_ERROR,
    };
    let mut bytes: &[u8] = std::slice::from_raw_parts(buffer, len);
    let params = match RecoverAccountParams::decode(&mut bytes) {
        Ok(params) => params,
        Err(_) => return BAD_PARAMETERS_ERROR,
    };
    let account = _recover_account(params, &root_seed);
    let mut buf = account.encode();
    let len = buf.len();
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);

    *out = ptr;
    *out_len = len;

    OKAY
}

#[no_mangle]
pub unsafe extern "C" fn get_private_transfer_batch_params_value(
    buffer: *const libc::c_uchar,
    len: libc::size_t,
    out: *mut *mut libc::c_char
) -> libc::size_t {
    let mut bytes: &[u8] = std::slice::from_raw_parts(buffer, len);
    let params_batch = match GeneratePrivateTransferBatchParams::decode(&mut bytes) {
        Ok(params_batch) => params_batch,
        Err(_) => return BAD_PARAMETERS_ERROR,
    };
    let value_string: String = _get_private_transfer_batch_params_value(params_batch);
    let value_string: CString = CString::new(value_string).expect("CString::new failed");

    *out = value_string.into_raw();

    OKAY
}

#[no_mangle]
pub unsafe extern "C" fn get_private_transfer_batch_params_currency_symbol(
    buffer: *const libc::c_uchar,
    len: libc::size_t,
    out: *mut *mut libc::c_char,
) -> libc::size_t {
    let mut bytes: &[u8] = std::slice::from_raw_parts(buffer, len);
    let params_batch = match GeneratePrivateTransferBatchParams::decode(&mut bytes) {
        Ok(params_batch) => params_batch,
        Err(_) => return BAD_PARAMETERS_ERROR,
    };
    let currency_symbol: String = match _get_private_transfer_batch_params_currency_symbol(params_batch) {
        Some(currency_symbol) => currency_symbol,
        None => return INVALID_ASSET_ID_ERROR,
    };
    let currency_symbol: CString = CString::new(currency_symbol).expect("CString::new failed");

    *out = currency_symbol.into_raw();

    OKAY
}

#[no_mangle]
pub unsafe extern "C" fn get_private_transfer_batch_params_recipient(
    buffer: *const libc::c_uchar,
    len: libc::size_t,
    out: *mut *mut libc::c_char
) -> libc::size_t {
    let mut bytes: &[u8] = std::slice::from_raw_parts(buffer, len);
    let params_batch = match GeneratePrivateTransferBatchParams::decode(&mut bytes) {
        Ok(params_batch) => params_batch,
        Err(_) => return BAD_PARAMETERS_ERROR,
    };
    let recipient: String = _get_private_transfer_batch_params_recipient(params_batch);
    let recipient: CString = CString::new(recipient).expect("CString::new failed");

    *out = recipient.into_raw();

    OKAY
}

#[no_mangle]
pub unsafe extern "C" fn get_reclaim_batch_params_currency_symbol(
    buffer: *const libc::c_uchar,
    len: libc::size_t,
    out: *mut *mut libc::c_char
) -> libc::size_t {
    let mut bytes: &[u8] = std::slice::from_raw_parts(buffer, len);
    let params_batch = match GenerateReclaimBatchParams::decode(&mut bytes) {
        Ok(params_batch) => params_batch,
        Err(_) => return BAD_PARAMETERS_ERROR,
    };
    let currency_symbol: String = match _get_reclaim_batch_params_currency_symbol(params_batch) {
        Some(currency_symbol) => currency_symbol,
        None => return INVALID_ASSET_ID_ERROR,
    };

    let currency_symbol: CString = CString::new(currency_symbol).expect("CString::new failed");

    *out = currency_symbol.into_raw();

    OKAY
}

#[no_mangle]
pub unsafe extern "C" fn get_reclaim_batch_params_value(
    buffer: *const libc::c_uchar,
    len: libc::size_t,
    out: *mut *mut libc::c_char,
) -> libc::size_t {
    let mut bytes: &[u8] = std::slice::from_raw_parts(buffer, len);
    let params_batch = match GenerateReclaimBatchParams::decode(&mut bytes) {
        Ok(params_batch) => params_batch,
        Err(_) => return BAD_PARAMETERS_ERROR,
    };
    let value_string: String = _get_reclaim_batch_params_value(params_batch);
    let value_string: CString = CString::new(value_string).expect("CString::new failed");

    *out = value_string.into_raw();

    OKAY
}

fn get_crypto_rng() -> ChaCha20Rng {
    let mut rng = thread_rng();
    let mut crypto_rng_seed = [0u8; 32];
    rng.fill_bytes(&mut crypto_rng_seed);

    ChaCha20Rng::from_seed(crypto_rng_seed)
}

fn deserialize_root_seed(
    root_seed: *const libc::c_uchar,
) -> Result<MantaRootSeed, <&'static [u8] as TryInto<MantaRootSeed>>::Error> {
    let bytes: &[u8] = unsafe { std::slice::from_raw_parts(root_seed, 64) };
    match bytes.try_into() {
        Ok(root_seed) => Ok(root_seed),
        Err(error) => Err(error),
    }
}
