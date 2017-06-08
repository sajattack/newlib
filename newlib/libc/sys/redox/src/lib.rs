#![no_std]
#![feature(collections, lang_items, core_intrinsics, compiler_builtins_lib, linkage)]

extern crate syscall;
#[macro_use]
extern crate collections;
extern crate compiler_builtins;

use core::{ptr, mem, intrinsics, slice};
use collections::Vec;
use ::types::{c_int, c_void, c_char, size_t};

#[macro_use]
mod macros;
pub mod process;
pub mod file;
pub mod folder;
pub mod time;
pub mod unimpl;
pub mod redox;
mod types;

extern {
    // Newlib uses this function instead of just a global to support reentrancy
    pub fn __errno() -> *mut c_int;
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn strlen(s: *const c_char) -> size_t;
    pub fn __libc_fini_array();
    pub static mut environ: *mut *mut c_char;
}

pub unsafe fn cstr_to_slice<'a>(buf: *const c_char) -> &'a [u8] {
    slice::from_raw_parts(buf as *const u8, ::strlen(buf) as usize)
}
pub unsafe fn cstr_to_slice_mut<'a>(buf: *const c_char) ->  &'a mut [u8] {
    slice::from_raw_parts_mut(buf as *mut u8, ::strlen(buf) as usize)
}

#[no_mangle]
pub unsafe extern "C" fn __errno_location() -> *mut c_int {
    __errno()
}

#[lang = "panic_fmt"]
#[linkage = "weak"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    unsafe { intrinsics::abort() }
}

libc_fn!(unsafe initialize_standard_library() {
    let fd = syscall::open("env:", syscall::O_RDONLY).unwrap();

    let mut vars = Vec::new();

    // XXX optimize
    let mut buf = [0; 1];
    'outer: loop {
        let mut line = Vec::new();
        loop {
            if syscall::read(fd, &mut buf).unwrap() == 0 {
                break 'outer;
            }
            if buf[0] == '\n' as u8 {
                break;
            }
            line.push(buf[0]);

        }
        line.push(0); // Null terminate
        vars.push(line.as_mut_ptr() as *mut c_char);
        mem::forget(line); // Do not free memory
    }
    vars.push(ptr::null_mut());

    environ = vars.as_mut_ptr();
    mem::forget(vars); // Do not free memory

    syscall::close(fd).unwrap();
});
