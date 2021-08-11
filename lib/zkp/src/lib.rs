extern crate libc;

use core::str::FromStr;
use hex;
use manta_api::signer::hd_wallet::derive_shielded_address as _derive_shielded_address;
use manta_api::signer::payload_gen::generate_asset as _generate_asset;

use manta_asset::MantaSecretKey;
use manta_crypto::MantaSerDes;
use std::ffi::{CStr, CString};
use tiny_hderive::bip44::DerivationPath;

#[no_mangle]
pub extern "C" fn generate_reclaim_data(
    asset_id: u32,
    asset_1_value: *const libc::c_char,
    asset_2_value: *const libc::c_char,

    _buffer: *const libc::c_char,
    _len: libc::size_t,
    _out: *mut *mut libc::c_char,
    out_len: *mut libc::size_t,
) -> libc::c_int {
    unsafe {
        let a = String::from("0xgenerate_transfer");
        let len = a.len();
        let s = CString::new(a).expect("CString::new failed");
        *_out = s.into_raw();
        *out_len = len;
    }
    0
}

#[no_mangle]
pub extern "C" fn generate_reclaim(
    _app_version: *const libc::c_char,
    _name: *const libc::c_char,
    _len: libc::size_t,
    _out: *mut *mut libc::c_char,
    out_len: *mut libc::size_t,
) -> libc::c_int {
    unsafe {
        let a = String::from("0xgenerate_reclaim");
        let len = a.len();
        let s = CString::new(a).expect("CString::new failed");
        *_out = s.into_raw();
        *out_len = len;
    }
    0
}

// todo: to_string for shielded address
#[no_mangle]
pub extern "C" fn derive_shielded_address(
    path: *const libc::c_char,
    asset_id: u32,
    out: *mut *mut libc::c_char,
    out_len: *mut libc::size_t,
) -> libc::c_int {
    unsafe {
        let path: &str = CStr::from_ptr(path).to_str().unwrap();
        let path: DerivationPath = DerivationPath::from_str(path).unwrap();
        let root_seed: MantaSecretKey = [0u8; 32].into();
        let shieded_address = _derive_shielded_address(path, asset_id, &root_seed);
        let mut buf: Vec<u8> = vec![];
        shieded_address.serialize(&mut buf).unwrap();
        let a = hex::encode(buf);
        let len = a.len();
        let s = CString::new(a).expect("CString::new failed");
        *out = s.into_raw();
        *out_len = len;
    }
    0
}

// todo: to_string for shielded address
#[no_mangle]
pub extern "C" fn generate_asset(
    asset_id: u32,
    value: *const libc::c_char,
    path: *const libc::c_char,
    out: *mut *mut libc::c_char,
    out_len: *mut libc::size_t,
) -> libc::c_int {
    unsafe {
        let value: &str = CStr::from_ptr(value).to_str().unwrap();
        let value: u128 = u128::from_str_radix(value, 10).unwrap();
        let path: &str = CStr::from_ptr(path).to_str().unwrap();
        let path: DerivationPath = DerivationPath::from_str(path).unwrap();
        let root_seed: MantaSecretKey = [0u8; 32].into();
        // todo: make ui safe asset type; private key is exposed here!!!
        let asset = _generate_asset(asset_id, value, path, &root_seed);
        let mut buf: Vec<u8> = vec![];
        asset.serialize(&mut buf).unwrap();
        let a = hex::encode(buf);
        let len = a.len();
        let s = CString::new(a).expect("CString::new failed");
        *out = s.into_raw();
        *out_len = len;
    }
    0
}

