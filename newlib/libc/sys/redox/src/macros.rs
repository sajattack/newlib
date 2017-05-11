use std::ptr;
use std::any::Any;
use libc::c_int;


extern {
    pub static mut errno: c_int;
}

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

impl Into<i8> for Fail {
    #[inline(always)]
    fn into(self) -> i8 {
        -1
    }
}

impl Into<i16> for Fail {
    #[inline(always)]
    fn into(self) -> i16 {
        -1
    }
}

impl Into<i32> for Fail {
    #[inline(always)]
    fn into(self) -> i32 {
        -1
    }
}

impl Into<i64> for Fail {
    #[inline(always)]
    fn into(self) -> i64 {
        -1
    }
}

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
