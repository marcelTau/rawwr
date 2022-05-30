use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::callable::*;
use crate::object::Object;
use crate::interpreter::Interpreter;
use crate::error::*;

pub struct NativeClock;

impl LoxCallable for NativeClock {
    fn call(&self, _interpreter: &Interpreter, _arguments: Vec<Object>) -> Result<Object, LoxError> {
        let sys_time = SystemTime::now();
        match sys_time.duration_since(UNIX_EPOCH) {
            Ok(t) => Ok(Object::Num(t.as_millis() as f64)),
            Err(e) => Err(LoxError::system_error("clock() failed."))
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
