use syscall::{self, O_CLOEXEC, O_STAT, O_CREAT, O_EXCL, O_DIRECTORY};
use core::slice;
use ::{c_int, c_char, off_t, mode_t};


libc_fn!(unsafe access(path: *mut c_char, _amode: c_int) -> Result<c_int> {
    // XXX amode
    let fd = syscall::open(::cstr_to_slice(path), O_CLOEXEC | O_STAT)?;
    syscall::close(fd)?;
    Ok(0)
});

libc_fn!(unsafe _close(file: c_int) -> Result<c_int> {
    Ok(syscall::close(file as usize)? as c_int)
});

libc_fn!(unsafe dup(file: c_int) -> Result<c_int> {
    Ok(syscall::dup(file as usize, &[])? as c_int)
});

libc_fn!(unsafe dup2(file: c_int, newfile: c_int) -> Result<c_int> {
    Ok(syscall::dup2(file as usize, newfile as usize, &[])? as c_int)
});

libc_fn!(unsafe _fpath(file: c_int, buf: *mut c_char, len: c_int) -> Result<c_int> {
    let buf = slice::from_raw_parts_mut(buf as *mut u8, len as usize);
    Ok(syscall::fpath(file as usize, buf)? as c_int)
});

libc_fn!(unsafe _fstat(file: c_int, st: *mut syscall::Stat) -> Result<c_int> {
    Ok(syscall::fstat(file as usize, &mut *st)? as c_int)
});

libc_fn!(unsafe _fsync(file: c_int) -> Result<c_int> {
    Ok(syscall::fsync(file as usize)? as c_int)
});

libc_fn!(unsafe ftruncate(file: c_int, len: off_t) -> Result<c_int> {
    Ok(syscall::ftruncate(file as usize, len as usize)? as c_int)
});

libc_fn!(unsafe _lseek(file: c_int, ptr: c_int, dir: c_int) -> Result<c_int> {
    Ok(syscall::lseek(file as usize, ptr as isize, dir as usize)? as c_int)
});


libc_fn!(unsafe mkdir(path: *mut c_char, mode: mode_t) -> Result<c_int> {
    let flags = O_CREAT | O_EXCL | O_CLOEXEC | O_DIRECTORY | (mode as usize & 0o777);
    let fd = syscall::open(::cstr_to_slice(path), flags)?;
    syscall::close(fd)?;
    Ok(0)
});

libc_fn!(unsafe _open(path: *mut c_char, flags: c_int, mode: mode_t) -> Result<c_int> {
    let path = ::cstr_to_slice(path);
    Ok(syscall::open(path, flags as usize | (mode as usize & 0o777))? as c_int)
});

libc_fn!(unsafe pipe(pipefd: *mut [c_int; 2]) -> c_int {
    pipe2(pipefd, 0)
});

libc_fn!(unsafe pipe2(pipefd: *mut [c_int; 2], flags: c_int) -> Result<c_int> {
    let mut syspipefd = [(*pipefd)[0] as usize, (*pipefd)[1] as usize];
    syscall::pipe2(&mut syspipefd, flags as usize)?;
    (*pipefd)[0] = syspipefd[0] as c_int;
    (*pipefd)[1] = syspipefd[1] as c_int;
    Ok(0)
});

libc_fn!(unsafe _read(file: c_int, buf: *mut c_char, len: c_int) -> Result<c_int> {
    let buf = slice::from_raw_parts_mut(buf as *mut u8, len as usize);
    Ok(syscall::read(file as usize, buf)? as c_int)
});

libc_fn!(unsafe rmdir(path: *mut c_char) -> Result<c_int> {
    Ok(syscall::rmdir(::cstr_to_slice(path))? as c_int)
});

libc_fn!(unsafe _stat(path: *const c_char, st: *mut syscall::Stat) -> Result<c_int> {
    let fd = syscall::open(::cstr_to_slice(path), O_CLOEXEC | O_STAT)?;
    let ret = _fstat(fd as c_int, st);
    let _ = syscall::close(fd);
    Ok(ret)
});

libc_fn!(unsafe _unlink(path: *mut c_char) -> Result<c_int> {
    Ok(syscall::unlink(::cstr_to_slice(path))? as c_int)
});

libc_fn!(unsafe _write(file: c_int, buf: *const c_char, len: c_int) -> Result<c_int> {
    let buf = slice::from_raw_parts(buf as *const u8, len as usize);
    Ok(syscall::write(file as usize, buf)? as c_int)
});

libc_fn!(unsafe chmod(path: *mut c_char, mode: mode_t) -> Result<c_int> {
    Ok(syscall::chmod(::cstr_to_slice(path), mode as usize)? as c_int)
});
