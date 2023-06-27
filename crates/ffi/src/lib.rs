extern crate libc;

const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");

#[no_mangle]
/// AnimBoom Toolkit version
pub extern "C" fn ABT_get_version() -> *const libc::c_char {
    VERSION.as_ptr().cast()
}
