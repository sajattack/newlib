use ::{c_int, c_char};
use syscall::{self, O_CLOEXEC, O_RDONLY, O_DIRECTORY};
use core::ptr::null;
use core::slice;
use collections::boxed::Box;

#[repr(C)]
pub struct dirent {
    pub d_name: [c_char; 4096],
}

#[repr(C)]
pub struct DIR {
    pub dd_fd: c_int, /* directory file */
    pub dd_ent: dirent,
}

libc_fn!(unsafe opendir(path: *mut c_char) -> Result<*mut DIR> {
    let path = ::cstr_to_slice(path);
    let fd = syscall::open(path, O_RDONLY | O_CLOEXEC | O_DIRECTORY)?;
    let dir = Box::new(DIR{dd_fd: fd as c_int, dd_ent: dirent{d_name: [0; 4096]}});
    Ok(Box::into_raw(dir))
});

libc_fn!(unsafe readdir(dir: *mut DIR) -> Result<*const dirent> {
    if !dir.is_null() {
        // TODO: Speed improvements
        let mut i = 0;
        while i < 3096 {
            let c = &mut (*dir).dd_ent.d_name[i];
            let buf = slice::from_raw_parts_mut(c as *mut i8 as *mut u8, 1);
            if syscall::read((*dir).dd_fd as usize, buf)? > 0 {
                if *c as u8 as char == '\n' {
                    break;
                }
            } else {
                break;
            }
            i += 1;
        }
        (*dir).dd_ent.d_name[i] = 0;
        if i > 0 {
            return Ok(&(*dir).dd_ent);
        }
    }
    Ok(null())
});

libc_fn!(unsafe rewinddir(dir: *const DIR) {
    if !(dir).is_null() {
        ::file::_lseek((*dir).dd_fd, 0, 0);
    }
});

libc_fn!(unsafe closedir(dir: *mut DIR) -> Result<c_int> {
    let ret = syscall::close((*dir).dd_fd as usize);
    Box::from_raw(dir);
    ret?;
    Ok(0)
});
