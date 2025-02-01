use libc::{c_int, c_void, size_t};

extern "C" {
    pub fn sysconf(name: c_int) -> c_long;
    pub fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: libc::off_t,
    ) -> *mut c_void;
    pub fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    pub fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int;
}

pub const _SC_PAGESIZE: c_int = 30; // Remplacez 30 par une valeur correcte si nécessaire.
pub const PROT_READ: c_int = 0x1;
pub const PROT_WRITE: c_int = 0x2;
pub const PROT_EXEC: c_int = 0x4;
pub const MAP_PRIVATE: c_int = 0x02;
pub const MAP_ANONYMOUS: c_int = 0x20; // Certaines plateformes utilisent MAP_ANON à la place.
pub const MAP_FAILED: *mut c_void = !0 as *mut c_void;