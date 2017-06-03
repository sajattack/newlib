extern crate syscall;
extern crate libc;

use libc::{c_int, c_void, c_char, size_t};
use std::{env, ptr, mem};
use std::ffi::CString;

#[macro_use]
mod macros;
pub mod process;
pub mod file;
pub mod folder;
pub mod time;
pub mod unimpl;

extern {
    // Newlib uses this function instead of just a global to support reentrancy
    pub fn __errno() -> *mut c_int;
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn strlen(s: *const c_char) -> size_t;
    pub fn __libc_fini_array();
    pub static mut environ: *mut *mut c_char;
}

#[no_mangle]
pub unsafe extern "C" fn __errno_location() -> *mut c_int {
    __errno()
}

libc_fn!(unsafe initialize_standard_library() {
    let mut vars = Vec::new();
    for (key, value) in env::vars() {
        vars.push(CString::from_vec_unchecked(format!("{}={}", key, value).into_bytes()).into_raw());
    }
    vars.push(ptr::null_mut());
    environ = vars.as_mut_ptr();
    mem::forget(vars); // Do not free memory
});
