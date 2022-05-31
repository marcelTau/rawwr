use crate::error::*;
use crate::interpreter::Interpreter;
use crate::object::Object;

use std::rc::Rc;
use core::fmt;

pub trait LoxCallable: fmt::Display {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult>;
    fn arity(&self) -> usize;
}

#[derive(Clone)]
pub struct Callable {
    pub func: Rc<dyn LoxCallable>,
}

impl LoxCallable for Callable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult> {
        self.func.call(interpreter, arguments)
    }

    fn arity(&self) -> usize {
        self.func.arity()
    }
}

impl fmt::Display for Callable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.func)
    }
}
impl fmt::Debug for Callable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.func)
    }
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.func, &other.func)
    }
}
