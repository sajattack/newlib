extern crate syscall;
extern crate libc;

use libc::c_int;

#[macro_use]
mod macros;
pub mod process;

extern {
    static mut errno: c_int;
}

#[no_mangle]
pub unsafe extern "C" fn __errno_location() -> *mut i32 {
    &mut errno as *mut i32
}
