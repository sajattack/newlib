use std::ffi::CStr;
use libc::{c_char, c_int, c_void, size_t, pid_t, gid_t, uid_t};
use std::slice;
use std::ptr::null;
use syscall::error::{Error, EINVAL};
use syscall;
use ::malloc;

const MAXPATHLEN: usize = 1024;

libc_fn!(unsafe chdir(path: *const c_char) -> c_int {
    Ok(syscall::chdir(&CStr::from_ptr(path).to_string_lossy())? as c_int)
});

libc_fn!(unsafe _exit(code: c_int) -> c_int {
    Ok(syscall::exit(code as usize)? as c_int)
});

libc_fn!(unsafe _execve(name: *const c_char, argv: *const *const c_char, _env: *const *const c_char) -> c_int {
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

libc_fn!(unsafe getwd(buf: *mut c_char) -> *const c_char {

    if buf.is_null() {
        Err(Error::new(EINVAL))
    } else {
        let mut tmp: [u8; MAXPATHLEN] = [0; MAXPATHLEN];
        syscall::getcwd(&mut tmp)?;
        slice::from_raw_parts_mut(buf as *mut u8, MAXPATHLEN)
            .copy_from_slice(&mut tmp);
        Ok(buf)
    }
});

// Cannot use libc_fn! since these functions return unsigned integers
#[no_mangle]
pub unsafe extern "C" fn _getpid() -> pid_t {
    syscall::getpid().unwrap() as pid_t
}

#[no_mangle]
pub unsafe extern "C" fn getegid() -> gid_t {
    syscall::getegid().unwrap() as gid_t
}

#[no_mangle]
pub unsafe extern "C" fn geteuid() -> uid_t {
    syscall::geteuid().unwrap() as uid_t
}

#[no_mangle]
pub unsafe extern "C" fn getgid() -> gid_t {
    syscall::getgid().unwrap() as gid_t
}

#[no_mangle]
pub unsafe extern "C" fn getuid() -> uid_t {
    syscall::getuid().unwrap() as uid_t
}

libc_fn!(unsafe _kill(pid: c_int, sig: c_int) -> c_int {
    Ok(syscall::kill(pid as usize, sig as usize)? as c_int)
});

libc_fn!(unsafe __brk(addr: *mut c_void) -> *mut c_void {
    Ok(syscall::brk(addr as usize)? as *mut c_void)

});
