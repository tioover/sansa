use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::convert::AsRef;
use std::collections::BTreeMap;
use glium::Display;


pub type Map<T> = BTreeMap<PathBuf, Weak<T>>;


pub trait Resource {
    fn load(&Display, &Path) -> Self;
}


pub struct Manager<'a, T: Resource> {
    pub map: RefCell<Map<T>>,
    path: PathBuf,
    display: &'a Display,
}


impl<'a, T: Resource> Manager<'a, T> {
    pub fn new(display: &'a Display, path: PathBuf) -> Manager<'a, T> {
        Manager {
            map: RefCell::new(Map::new()),
            path: path,
            display: display,
        }
    }

    fn load(&self, key: &PathBuf) -> Rc<T> {
        let mut map = self.map.borrow_mut();
        if map.contains_key(key) { let _ = map.remove(key); }
        let res = Rc::new(T::load(self.display, &key));
        map.insert(key.clone(), Rc::downgrade(&res));
        return res;
    }

    pub fn get<P: AsRef<Path>>(&self, p: P) -> Rc<T> {
        let key = self.path.join(p);
        match {
            let map = self.map.borrow();
            if map.contains_key(&key) {
                let weak = self.map.borrow()[&key].clone();
                weak.upgrade()
            }
            else {
                None
            }
        } {
            None => self.load(&key),
            Some (x) => x,
        }
    }
}

