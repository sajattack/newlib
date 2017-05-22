use std::ffi::CStr;
use libc::{c_char, c_int, c_void, size_t, pid_t, gid_t, uid_t, ptrdiff_t};
use std::slice;
use std::ptr::null;
use syscall::error::{Error, EINVAL};
use syscall;
use ::malloc;

const MAXPATHLEN: usize = 1024;
static mut CURR_BRK: usize = 0;

libc_fn!(unsafe chdir(path: *const c_char) -> Result<c_int> {
    Ok(syscall::chdir(&CStr::from_ptr(path).to_string_lossy())? as c_int)
});

libc_fn!(unsafe _exit(code: c_int) -> Result<c_int> {
    Ok(syscall::exit(code as usize)? as c_int)
});

libc_fn!(unsafe _execve(name: *const c_char, argv: *const *const c_char, _env: *const *const c_char) -> Result<c_int> {
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

libc_fn!(unsafe _fork() -> Result<c_int> {
    Ok(syscall::clone(0)? as c_int)
});

libc_fn!(unsafe getcwd(buf: *mut c_char, size: size_t) -> Result<*const c_char> {
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

libc_fn!(unsafe getwd(buf: *mut c_char) -> Result<*const c_char> {

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

libc_fn!(unsafe _getpid() -> pid_t {
    syscall::getpid().unwrap() as pid_t
});

libc_fn!(unsafe getegid() -> gid_t {
    syscall::getegid().unwrap() as gid_t
});

libc_fn!(unsafe geteuid() -> uid_t {
    syscall::geteuid().unwrap() as uid_t
});

libc_fn!(unsafe getgid() -> gid_t {
    syscall::getgid().unwrap() as gid_t
});

libc_fn!(unsafe getuid() -> uid_t {
    syscall::getuid().unwrap() as uid_t
});

libc_fn!(unsafe _kill(pid: c_int, sig: c_int) -> Result<c_int> {
    Ok(syscall::kill(pid as usize, sig as usize)? as c_int)
});

libc_fn!(unsafe _brk(end_data_segment: *mut c_void) -> Result<c_int> {
    CURR_BRK = syscall::brk(end_data_segment as usize)?;
    Ok(0)
});

libc_fn!(unsafe _sbrk(increment: ptrdiff_t) -> Result<*mut c_void> {
    if CURR_BRK == 0 {
        CURR_BRK = syscall::brk(0)?;
    }
    let old_brk = CURR_BRK;
    CURR_BRK = syscall::brk(CURR_BRK + increment as usize)?;
    Ok(old_brk as *mut c_void)
});

libc_fn!(unsafe _sched_yield() -> Result<c_int> {
    Ok(syscall::sched_yield()? as c_int)
});

libc_fn!(unsafe _system(s: *const c_char) -> Result<c_int> {
    match syscall::clone(0)? {
        0 => {
            let arg1 = "-c";
            let args = [
                [arg1.as_ptr() as usize, arg1.len()],
                [s as usize, ::strlen(s)]
            ];
            syscall::execve("/bin/sh", &args)?;
            syscall::exit(100)?;
            unreachable!()
        }
        pid => {
            let mut status = 0;
            syscall::waitpid(pid, &mut status, 0)?;
            Ok(status as c_int)
        }
    }
});

libc_fn!(unsafe setregid(rgid: gid_t, egid: gid_t) -> Result<c_int> {
    Ok(syscall::setregid(rgid as usize, egid as usize)? as c_int)
});

libc_fn!(unsafe setreuid(ruid: uid_t, euid: uid_t) -> Result<c_int> {
    Ok(syscall::setregid(ruid as usize, euid as usize)? as c_int)
});

libc_fn!(unsafe _wait(status: *mut c_int) -> Result<c_int> {
    let mut buf = 0;
    let res = syscall::waitpid(0, &mut buf, 0)?;
    *status = buf as c_int;
    Ok(res as c_int)
});

libc_fn!(unsafe waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> Result<c_int> {
    let mut buf = 0;
    let res = syscall::waitpid(pid as usize, &mut buf, options as usize)?;
    *status = buf as c_int;
    Ok(res as c_int)
});
