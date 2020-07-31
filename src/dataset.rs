use super::native;
use super::pool::Pool;
use std::ffi::CStr;

struct Handle {
    ptr: *mut native::zfs_handle_t
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { native::zfs_close(self.ptr) }
    }
}

pub struct Dataset {
    h: Handle,
}

impl Dataset {
    // Marked unsafe as we have no real guarantees the pointer really "belongs" to us
    pub(crate) unsafe fn from_ptr(ptr: *mut native::zfs_handle_t) -> Dataset {
        Dataset { h: Handle { ptr } }
    }

    pub fn name(&self) -> String {
        let c_name = unsafe { CStr::from_ptr(native::zfs_get_name(self.h.ptr)) };
        c_name.to_str().unwrap().into()
    }

    pub fn pool(&self) -> Result<Pool, ()> {
        let pool_name = unsafe { native::zfs_get_pool_name(self.h.ptr) };
        let zfs = unsafe { native::zfs_get_handle(self.h.ptr) };
        let pool = unsafe { native::zpool_open(zfs, pool_name) };

        if pool.is_null() {
            Err(())
        } else {
            Ok(unsafe { Pool::from_ptr(pool) })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Zfs;

    #[test]
    fn test_pool() {
        let zfs = Zfs::new().unwrap();
        let dataset = zfs.dataset("macpro/home").unwrap();
        let pool = dataset.pool().unwrap();

        assert_eq!(&pool.name(), "macpro");
    }
}
