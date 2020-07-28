use std::os::raw;

#[repr(C)]
pub struct libzfs_handle_t {
    _unused: [u8; 0]
}

#[repr(C)]
pub struct zfs_handle_t {
    _unused: [u8; 0]
}

#[allow(non_camel_case_types)]
pub type zfs_type_t = i32;

#[allow(dead_code)]
pub mod zfs_type {
    use super::zfs_type_t;

    pub const FILESYSTEM: zfs_type_t = 1;
    pub const SNAPSHOT: zfs_type_t = 2;
    pub const VOLUME: zfs_type_t = 4;
    pub const POOL: zfs_type_t = 8;
    pub const BOOKMARK: zfs_type_t = 16;
}

#[link(name = "zfs")]
extern "C" {
    pub fn libzfs_init() -> *mut libzfs_handle_t;
    pub fn libzfs_fini(_: *mut libzfs_handle_t);

    pub fn zfs_open(_: *mut libzfs_handle_t, _: *const raw::c_char, _: raw::c_int) -> *mut zfs_handle_t;
    pub fn zfs_close(_: *mut zfs_handle_t);
    pub fn zfs_get_name(_: *mut zfs_handle_t) -> *const raw::c_char;
}
