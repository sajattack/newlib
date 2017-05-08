extern crate syscall;
extern crate libc;

use std::ffi::CStr;
use libc::{c_char, c_int, c_void, size_t};
use std::slice;
use syscall::{O_CLOEXEC, O_STAT};
use std::ptr::null;

extern {
    static mut errno: c_int;
    fn malloc(size: size_t) -> *mut c_void;
}

macro_rules! try_call {
    ($res:expr) => (
        match $res {
            Ok(val) => val,
            Err(err) => {
                errno = err.errno;
                return -1;
            }
        }
    );
}

macro_rules! try_call_ptr {
    ($res:expr) => (
        match $res {
            Ok(val) => val,
            Err(err) => {
                errno = err.errno;
                return null();
            }
        }
    );
}

#[no_mangle]
pub unsafe extern "C" fn __errno_location() -> *mut i32 {
    &mut errno as *mut i32
}

#[no_mangle]
pub unsafe extern "C" fn chdir(path: *const c_char) -> c_int {
    try_call!(syscall::chdir(&CStr::from_ptr(path).to_string_lossy())) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn _execve(name: *const c_char, argv: *const *const c_char, env: *const *const c_char) -> c_int {
    // XXX Handle env
    
    let mut args: Vec<[usize; 2]> = Vec::new();
    let mut arg = argv;
    while !(*arg).is_null() {
        args.push([*arg as usize, CStr::from_ptr(*arg).to_bytes().len()]);
        arg = arg.offset(1);
    }

    let name = CStr::from_ptr(name).to_string_lossy();

    try_call!(syscall::execve(&name, &args)) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn _fork() -> c_int {
    try_call!(syscall::clone(0)) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn _getcwd(mut buf: *mut c_char, mut size: size_t) -> *const c_char {
    if size == 0 {
        size = 4096;
    }
    if buf.is_null() {
        buf = malloc(size) as *mut c_char;
        if buf.is_null() {
            return null();
        }
    } 
    let slice = slice::from_raw_parts_mut(buf as *mut u8, size);
    let fd = try_call_ptr!(syscall::open(".", O_CLOEXEC | O_STAT));
    try_call_ptr!(syscall::fpath(fd, slice));
    try_call_ptr!(syscall::close(fd));
    buf
    // FIXME: buffer too small; free buf when allocated
}
