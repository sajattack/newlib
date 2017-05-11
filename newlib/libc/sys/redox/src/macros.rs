use std::ptr;
use std::any::Any;
use libc::c_int;


extern {
    pub static mut errno: c_int;
}

/// This struct converts to `NULL` for raw pointers, and `-1` for signed
/// integers.
pub struct Fail;

impl<T: Any> Into<*const T> for Fail {
    #[inline(always)]
    fn into(self) -> *const T {
        ptr::null()
    }
}

impl<T: Any> Into<*mut T> for Fail {
    #[inline(always)]
    fn into(self) -> *mut T {
        ptr::null_mut()
    }
}

macro_rules! int_fail {
    ($type:ty) => (
        impl Into<$type> for Fail {
            #[inline(always)]
            fn into(self) -> $type {
                -1
            }
        }
    )
}

int_fail!(i8);
int_fail!(i16);
int_fail!(i32);
int_fail!(i64);

/// If `res` is `Err(..)`, set `errno` and return `-1` or `NULL`, otherwise
/// unwrap.
macro_rules! try_call {
    ($res:expr) => (
        match $res {
            Ok(val) => val,
            Err(err) => {
                ::macros::errno = err.errno;
                return ::macros::Fail.into();
            }
        }
    );
}

/// Declares a libc function. The body should return syscall::Result, which
/// is used to set errno on error with try_call!
///
/// ```
/// libc_fn!(foo(arg: c_int) -> c_int) {
///     Ok(arg)
/// }
/// ```
///
/// The `unsafe` keyword can be added to make the function unsafe:
///
/// ```
/// libc_fn!(unsafe foo(arg: c_int) -> c_int) {
///     Ok(arg)
/// }
/// ```
macro_rules! libc_fn {
    ($name:ident($($aname:ident : $atype:ty),+) -> $rtype:ty $content:block) => {
        #[no_mangle]
        pub extern "C" fn $name($($aname: $atype,)+) -> $rtype {
            #[inline(always)]
            fn internal($($aname: $atype,)+) -> ::syscall::Result<$rtype> {
                $content
            }
            unsafe { try_call!(internal($($aname,)+)) }
        }
    };
    ($name:ident() -> $rtype:ty $content:block) => {
        #[no_mangle]
        pub extern "C" fn $name() -> $rtype {
            #[inline(always)]
            fn internal() -> ::syscall::Result<$rtype> {
                $content
            }
            unsafe { try_call!(internal()) }
        }
    };
    (unsafe $name:ident($($aname:ident : $atype:ty),+) -> $rtype:ty $content:block) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name($($aname: $atype,)+) -> $rtype {
            #[inline(always)]
            unsafe fn internal($($aname: $atype,)+) -> ::syscall::Result<$rtype> {
                $content
            }
            try_call!(internal($($aname,)+))
        }
    };
    (unsafe $name:ident() -> $rtype:ty $content:block) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name() -> $rtype {
            #[inline(always)]
            unsafe fn internal() -> ::syscall::Result<$rtype> {
                $content
            }
            try_call!(internal())
        }
    };
    ($name:ident($($aname:ident : $atype:ty),+) $content:block) => {
        #[no_mangle]
        pub extern "C" fn $name($($aname: $atype,)+) {
            $content
        }
    };
    ($name:ident() -> $content:block) => {
        #[no_mangle]
        pub extern "C" fn $name() {
            $content
        }
    };
    (unsafe $name:ident($($aname:ident : $atype:ty),+) $content:block) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name($($aname: $atype,)+) {
            $content
        }
    };
    (unsafe $name:ident() $content:block) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name() {
            $content
        }
    };
}
