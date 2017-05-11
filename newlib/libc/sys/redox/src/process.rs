use std::ffi::CStr;
use libc::{c_char, c_int, size_t};
use std::slice;
use std::ptr::null;
use syscall;
use ::malloc;


libc_fn!(unsafe chdir(path: *const c_char) -> c_int {
    Ok(syscall::chdir(&CStr::from_ptr(path).to_string_lossy())? as c_int)
});

libc_fn!(unsafe _execve(name: *const c_char, argv: *const *const c_char, env: *const *const c_char) -> c_int {
    // XXX Handle env
    
    let mut args: Vec<[usize; 2]> = Vec::new();
    let mut arg = argv;
    while !(*arg).is_null() {
        args.push([*arg as usize, CStr::from_ptr(*arg).to_bytes().len()]);
        arg = arg.offset(1);
    }

    let name = CStr::from_ptr(name).to_string_lossy();

    Ok(syscall::execve(&name, &args)? as c_int)
});

libc_fn!(unsafe _fork() -> c_int {
    Ok(syscall::clone(0)? as c_int)
});

libc_fn!(unsafe _getcwd(buf: *mut c_char, size: size_t) -> *const c_char {
    let mut buf = buf;
    let mut size = size;
    if size == 0 {
        size = 4096;
    }
    if buf.is_null() {
        buf = malloc(size) as *mut c_char;
        if buf.is_null() {
            return Ok(null());
        }
    } 
    syscall::getcwd(slice::from_raw_parts_mut(buf as *mut u8, size))?;
    Ok(buf)
    // FIXME: buffer too small; free buf when allocated
});
