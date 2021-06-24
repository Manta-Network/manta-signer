extern crate libc;

#[no_mangle]
pub extern "C" fn hello(_name: *const libc::c_char) -> *const libc::c_char {
    return _name;
}