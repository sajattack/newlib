use core::str::{self, FromStr};
use ::types::{c_int, c_char, size_t, c_void, ssize_t, socklen_t, in_addr, sockaddr, sockaddr_in};
use syscall::{self, O_RDWR};
use syscall::error::{Error, EPROTOTYPE, EPROTONOSUPPORT, EAFNOSUPPORT, EINVAL, EOPNOTSUPP};
use core::slice;
use collections::Vec;
use byteorder::{BigEndian, ByteOrder};


const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;

static mut NTOA_ADDR: Option<Vec<u8>> = None;

libc_fn!(unsafe inet_aton(cp: *const c_char, inp: *mut in_addr) -> c_int {
    // TODO: octal/hex; more modern functions
    let addr = &mut (*inp).s_addr;
    let mut octets = str::from_utf8_unchecked(::cstr_to_slice(cp)).split('.');
    for i in 0..4 {
        if let Some(n) = octets.next().and_then(|x| u8::from_str(x).ok()) {
            addr[i] = n;
        } else {
            return 0;
        }
    }
    if octets.next() == None {
        1 // Success
    } else {
        0
    }
});

libc_fn!(unsafe inet_ntoa(inp: *const in_addr) -> *const c_char {
    let s_addr = (*inp).s_addr;
    let addr = format!("{}.{}.{}.{}\0", s_addr[0], s_addr[1], s_addr[2], s_addr[3]).into_bytes();
    let ptr = addr.as_ptr();
    NTOA_ADDR = Some(addr);
    ptr as *const c_char
});

libc_fn!(unsafe socket(domain: c_int, type_: c_int, protocol: c_int) -> Result<c_int> {
    if domain != AF_INET {
        Err(Error::new(EAFNOSUPPORT))
    } else if protocol != 0 {
        Err(Error::new(EPROTONOSUPPORT))
    } else {
        let fd = match type_ {
            SOCK_STREAM => syscall::open("tcp:", O_RDWR),
            SOCK_DGRAM => syscall::open("udp:", O_RDWR),
            _ => Err(Error::new(EPROTOTYPE))
        }?;
        Ok(fd as c_int)
    }
});

libc_fn!(unsafe connect(socket: c_int, address: *const sockaddr, _address_len: socklen_t) -> Result<c_int> {
    // XXX with UDP, should recieve messages only from that peer after this
    // XXX Check address_len
    if (*address).sa_family as i32 != AF_INET {
        return Err(Error::new(EINVAL))
    };
    let address = &*(address as *const sockaddr_in);
    let s_addr =address.sin_addr.s_addr;
    let path = format!("{}.{}.{}.{}:{}", s_addr[0], s_addr[1], s_addr[2], s_addr[3], ntohs(address.sin_port));
    let fd = syscall::dup(socket as usize, path.as_bytes())?;
    let ret = syscall::dup2(fd, socket as usize, &vec![]);
    let _ = syscall::close(fd);
    ret?;
    Ok(0)
});

libc_fn!(unsafe recv(socket: c_int, buffer: *mut c_void, length: size_t, flags: c_int) -> Result<ssize_t> {
    // XXX flags
    if flags != 0 {
        Err(Error::new(EOPNOTSUPP))
    } else {
        let buf = slice::from_raw_parts_mut(buffer as *mut u8, length);
        Ok(syscall::read(socket as usize, buf)? as ssize_t)
    }
});

libc_fn!(unsafe send(socket: c_int, buffer: *const c_void, length: size_t, flags: c_int) -> Result<ssize_t> {
    if flags != 0 {
        Err(Error::new(EOPNOTSUPP))
    } else {
        let buf = slice::from_raw_parts(buffer as *const u8, length);
        Ok(syscall::write(socket as usize, buf)? as ssize_t)
    }
});

libc_fn!(htonl(hostlong: u32) -> [u8; 4] {
    let mut netlong = [0; 4];
    BigEndian::write_u32(&mut netlong, hostlong);
    netlong
});

libc_fn!(htons(hostshort: u16) -> [u8; 2] {
    let mut netshort = [0; 2];
    BigEndian::write_u16(&mut netshort, hostshort);
    netshort
});

libc_fn!(ntohl(netlong: [u8; 4]) -> u32 {
    BigEndian::read_u32(&netlong)
});

libc_fn!(ntohs(netshort: [u8; 2]) -> u16 {
    BigEndian::read_u16(&netshort)
});
