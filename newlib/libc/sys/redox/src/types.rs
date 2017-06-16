#![allow(non_camel_case_types, dead_code)]

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
pub type pid_t = c_int;
pub type gid_t = usize;
pub type uid_t = usize;


// Socket related types
pub type in_addr_t = [u8; 4];
pub type sa_family_t = u16;
pub type socklen_t = size_t; 
pub type in_port_t = [u8; 2];

#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    sa_data: [c_char; 14]
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    __pad: [u8; 8]
}

#[repr(C)]
pub struct hostent {
    pub h_name: *const c_char,
    pub h_aliases: *const *const c_char,
    pub h_addrtype: c_int,
    pub h_length: c_int,
    pub h_addr_list: *const *const c_char
}
