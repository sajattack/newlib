#![no_std]
#![feature(collections, lang_items, core_intrinsics, compiler_builtins_lib, linkage)]
#![allow(non_camel_case_types)]

extern crate syscall;
#[macro_use]
extern crate collections;
extern crate compiler_builtins;

use core::{ptr, mem, intrinsics, slice};
use collections::Vec;

#[macro_use]
mod macros;
pub mod process;
pub mod file;
pub mod folder;
pub mod time;
pub mod unimpl;

// Copied from libc crate
#[repr(u8)]
pub enum c_void {
    // Two dummy variants so the #[repr] attribute can be used.
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_float = f32;
pub type c_double = f64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;

pub type c_char = i8;
pub type c_long = i64;
pub type c_ulong = u64;

pub type wchar_t = i16;

pub type off_t = usize;
pub type mode_t = u16;
pub type time_t = i64;
pub type pid_t = usize;
pub type gid_t = usize;
pub type uid_t = usize;

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
