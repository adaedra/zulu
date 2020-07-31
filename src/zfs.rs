use super::native;
use super::handle::Handle;
use super::dataset::Dataset;
use super::pool::Pool;

use std::rc::Rc;
use std::ffi::CString;

#[derive(Clone)]
pub struct Zfs {
    h: Rc<Handle>
}

impl Zfs {
    pub fn new() -> Result<Zfs, ()> {
        Ok(Zfs { h: Rc::new(Handle::new()?) })
    }

    pub fn dataset(&self, path: &str) -> Result<Dataset, ()> {
        let c_path = CString::new(path).unwrap();
        let ptr = unsafe { native::zfs_open(self.h.ptr, c_path.as_ptr(), native::zfs_type::FILESYSTEM) };

        if ptr.is_null() {
            Err(())
        } else {
            Ok(unsafe { Dataset::from_ptr(ptr) })
        }
    }

    pub fn pool(&self, name: &str) -> Result<Pool, ()> {
        let c_name: CString = CString::new(name).unwrap();
        let ptr = unsafe { native::zpool_open(self.h.ptr, c_name.as_ptr()) };

        if ptr.is_null() {
            Err(())
        } else {
            Ok(unsafe { Pool::from_ptr(ptr) })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Zfs;

    #[test]
    fn test_open_and_close() {
        Zfs::new().unwrap();
    }

    #[test]
    fn test_dataset() {
        let zfs = Zfs::new().unwrap();
        let dataset = zfs.dataset("macpro/home").unwrap();

        assert_eq!(&dataset.name(), "macpro/home");
    }

    #[test]
    fn test_pool() {
        let zfs = Zfs::new().unwrap();
        let pool = zfs.pool("macpro").unwrap();

        assert_eq!(&pool.name(), "macpro");
    }
}
