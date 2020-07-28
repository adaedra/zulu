use super::native;
use super::handle::Handle;
use super::dataset::Dataset;

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
}

#[cfg(test)]
mod tests {
    use super::Zfs;

    #[test]
    fn test_open_and_close() {
        Zfs::new().unwrap();
    }
}
