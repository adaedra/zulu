use super::native;
use std::ffi::CStr;

struct Handle {
    ptr: *mut native::zpool_handle_t
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { native::zpool_close(self.ptr) }
    }
}

pub struct Pool {
    h: Handle
}

impl Pool {
    // Marked unsafe as we have no real guarantees the pointer really "belongs" to us
    pub(crate) unsafe fn from_ptr(ptr: *mut native::zpool_handle_t) -> Pool {
        Pool { h: Handle { ptr } }
    }

    pub fn name(&self) -> String {
        let c_name = unsafe { CStr::from_ptr(native::zpool_get_name(self.h.ptr)) };
        c_name.to_str().unwrap().into()
    }
}
