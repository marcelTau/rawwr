use crate::error::*;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::class::*;

use core::fmt;
use std::ptr;
use std::rc::Rc;

pub trait LoxCallable: fmt::Display {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>, klass: Option<Rc<Class>>) -> Result<Object, LoxResult>;
    fn arity(&self) -> usize;
}

#[derive(Clone)]
pub struct Callable {
    pub func: Rc<dyn LoxCallable>,
}

impl LoxCallable for Callable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>, _klass: Option<Rc<Class>>) -> Result<Object, LoxResult> {
        self.func.call(interpreter, arguments, None)
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
        //ptr::eq(&self.func, &other.func) // @todo fix this
        //ptr::eq(self as *const _ as *const _ , other as *const _ as *const _) // @todo fix this
        ptr::eq(
            Rc::as_ptr(&self.func) as *const (),
            Rc::as_ptr(&other.func) as *const (),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_functions::*;
    use std::rc::Rc;

    #[test]
    fn test_equality_not_equal() {
        let f1 = NativeClock {};
        let f2 = NativeClock {};

        let c1 = Callable { func: Rc::new(f1) };
        let c2 = Callable { func: Rc::new(f2) };

        assert!(c1 != c2);
    }
}
