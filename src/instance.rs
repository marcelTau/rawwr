use std::fmt;
use std::rc::Rc;
use crate::class::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Instance {
    klass: Rc<Class>,
}

impl Instance {
    pub fn new(klass: Rc<Class>) -> Self {
        Self {
            klass: Rc::clone(&klass)
        }
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.klass)
    }
}
