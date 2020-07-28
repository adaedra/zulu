use super::native;

pub struct Handle {
    ptr: *mut native::libzfs_handle_t
}

impl Handle {
    pub fn new() -> Result<Handle, ()> {
        let ptr = unsafe { native::libzfs_init() };

        if ptr.is_null() {
            Err(())
        } else {
            Ok(Handle { ptr })
        }
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { native::libzfs_fini(self.ptr) }
    }
}
