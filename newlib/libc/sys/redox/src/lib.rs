extern crate syscall;
extern crate libc;

use libc::{c_int, c_void, size_t};

#[macro_use]
mod macros;
pub mod process;
pub mod file;

extern {
    pub static mut errno: c_int;
    pub fn malloc(size: size_t) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn __errno_location() -> *mut i32 {
    &mut errno as *mut i32
}
