extern crate libc;

use std::ffi::CString;

#[no_mangle]
pub extern "C" fn generate_transfer(
    _app_version: *const libc::c_char,
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
