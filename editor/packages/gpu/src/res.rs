use std::{ops::Deref, rc::Rc};

pub struct Res<T> {
    rc: Rc<T>,
}

impl<T> Res<T> {
    pub(crate) fn new(value: T) -> Self {
        Self { rc: Rc::new(value) }
    }

    pub fn share(&self) -> Self {
        Self {
            rc: self.rc.clone(),
        }
    }
}

impl<T> Deref for Res<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.rc.deref()
    }
}
