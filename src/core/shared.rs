use std::cell::RefCell;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct Shared<T>(Rc<RefCell<T>>);

impl<T> Shared<T> {
    #[must_use]
    pub fn new(value: T) -> Self {
        Shared(Rc::new(RefCell::new(value)))
    }
}

impl<T> Deref for Shared<T> {
    type Target = Rc<RefCell<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Shared<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Hash> Hash for Shared<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let inner = self.borrow();
        (*inner).hash(state);
    }
}

impl<T: Default> Default for Shared<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}
