#[repr(C)]
pub struct libzfs_handle_t {
    _unused: [u8; 0]
}

#[link(name = "zfs")]
extern "C" {
    pub fn libzfs_init() -> *mut libzfs_handle_t;
    pub fn libzfs_fini(_: *mut libzfs_handle_t);
}
