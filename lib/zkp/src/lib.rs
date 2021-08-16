extern crate libc;
extern crate alloc;

use std::ffi::{CString, CStr};
use std::ops::{Deref, DerefMut};
use alloc::slice;
use manta_asset::MantaSecretKey;
use manta_api::signer::desktop_app::hd_wallet::derive_shielded_address as _derive_shielded_address;
use manta_api::signer::desktop_app::payload_gen::generate_asset as _generate_asset;
use manta_api::signer::shared::params::{DeriveShieldedAddressParams, GenerateAssetParams};
use manta_crypto::MantaSerDes;

static mut PASSWORD: String = String::new();
static mut ACCOUNT_CREATED: bool = false;

struct Buffer {
    ptr: *mut libc::c_char,
    len: libc::size_t,
}

impl Buffer {
    pub unsafe fn from_owning(buffer: *mut libc::c_char, len: libc::size_t) -> Option<Self> {
        if buffer.is_null() || len >= usize::MAX as usize {
            None
        } else {
            Some(Buffer{
                ptr: buffer,
                len,
            })
        }
    }
}

impl Deref for Buffer {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        unsafe {
            slice::from_raw_parts(self.ptr as *const u8, self.len)
        }
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            slice::from_raw_parts_mut(self.ptr as *mut u8, self.len)
        }
    }
}


#[no_mangle]
pub extern "C" fn generate_transfer(
    _app_version: *const libc::c_char,
    buffer: *mut libc::c_char,
    len: libc::size_t,
    _out: *mut *mut libc::c_char,
    out_len: *mut libc::size_t) -> libc::c_int {
    unsafe {
        let buf = Buffer::from_owning(buffer, len).unwrap();
        let digest = md5::compute(buf.deref());
        let a = format!("{:x}", digest);
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
    out_len: *mut libc::size_t) -> libc::c_int {
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
        let root_seed: MantaSecretKey = [0u8; 32].into();
        let params = DeriveShieldedAddressParams {
            path: String::from(path),
            asset_id
        };

        let shielded_address = _derive_shielded_address(params, &root_seed);
        let mut buf: Vec<u8> = vec![];
        shielded_address.serialize(&mut buf).unwrap();
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
        let root_seed: MantaSecretKey = [0u8; 32].into();
        let params = GenerateAssetParams{
            asset_id,
            value,
            path: String::from(path),
        };
        // todo: make ui safe asset type; private key is exposed here!!!
        let asset = _generate_asset(params, &root_seed);
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

#[no_mangle]
pub extern "C" fn generate_recovery_phrase(
    password: *const libc::c_char
) -> *mut libc::c_char {
    unsafe {
        // todo this should to done by rust team to generate secret key by recovery phrase and password
        ACCOUNT_CREATED = true;
        PASSWORD = CStr::from_ptr(password).to_str().unwrap().to_string();
        let s = CString::new("wealth enrich manual process trap issue olympic stand gravity luggage tissue soon").expect("CString::new failed");
        return s.into_raw();
    }
}

#[no_mangle]
pub extern "C" fn modify_password_by_recovery_phrase(
    _recovery_phrase: *const libc::c_char,
    password: *const libc::c_char,
) -> libc::c_int {
    unsafe {
        // todo this function only return OK/ERR
        PASSWORD = CStr::from_ptr(password).to_str().unwrap().to_string();
    }
    0
}

#[no_mangle]
pub extern "C" fn verify_password(
    password: *const libc::c_char,
) -> libc::c_int {
    unsafe {
        return if CStr::from_ptr(password).to_str().unwrap().to_string() == PASSWORD {
            0
        } else {
            1
        }
    }
}

#[no_mangle]
pub extern "C" fn account_created(
) -> libc::c_int {
    unsafe {
        if ACCOUNT_CREATED {
            1
        } else {
            0
        }
    }
}