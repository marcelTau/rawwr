use std::fmt;
use std::ptr;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::callable::*;
use crate::object::Object;
use crate::interpreter::Interpreter;
use crate::error::*;
use crate::class::*;

#[derive(Clone)]
pub struct Native {
    pub func: Rc<dyn LoxCallable>,
}

impl fmt::Display for Native {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Native function>")
    }
}

impl fmt::Debug for Native {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Native function>")
    }
}

impl PartialEq for Native {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(
            Rc::as_ptr(&self.func) as *const (),
            Rc::as_ptr(&other.func) as *const (),
        )
    } 
}


pub struct NativeClock;

impl LoxCallable for NativeClock {
    fn call(&self, _interpreter: &Interpreter, _arguments: Vec<Object>, klass: Option<Rc<Class>>) -> Result<Object, LoxResult> {
        let sys_time = SystemTime::now();
        match sys_time.duration_since(UNIX_EPOCH) {
            Ok(t) => Ok(Object::Num(t.as_millis() as f64)),
            Err(e) => Err(LoxResult::system_error("clock() failed."))
        }
    }

    fn arity(&self) -> usize {
        0
    }
}

impl fmt::Display for NativeClock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<native function>")
    }
}


pub struct NativeNumToString;

impl LoxCallable for NativeNumToString {
    fn call(&self, _interpreter: &Interpreter, arguments: Vec<Object>, klass: Option<Rc<Class>>) -> Result<Object, LoxResult> {
        Ok(Object::Str(format!("{}", arguments[0])))
    }

    fn arity(&self) -> usize {
        1
    }
}

impl fmt::Display for NativeNumToString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<native function>")
    }
}
