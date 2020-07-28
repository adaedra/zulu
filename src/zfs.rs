use super::handle::Handle;
use std::rc::Rc;

#[derive(Clone)]
pub struct Zfs {
    h: Rc<Handle>
}

impl Zfs {
    pub fn new() -> Result<Zfs, ()> {
        Ok(Zfs { h: Rc::new(Handle::new()?) })
    }
}
