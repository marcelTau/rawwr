use std::fmt;
use std::rc::Rc;
use crate::callable::*;
use crate::error::*;
use crate::instance::Instance;
use crate::object::*;
use crate::interpreter::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Class {
    name: String,
}

impl Class {
    pub fn new(name: String)  -> Self {
        Self {
            name
        }
    }

    pub fn instantiate(&self, interpreter: &Interpreter, arguments: Vec<Object>, klass: Rc<Class>) -> Result<Object, LoxResult> {
        Ok(Object::Instance(Instance::new(klass)))
    }
}

//impl std::string::ToString for Class {
    //fn to_string(&self) -> String {
        //self.name.clone()
    //}
//}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl LoxCallable for Class {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult> {
        Ok(Object::Instance(Instance::new(Rc::new(self.clone()))))
    }
    fn arity(&self) -> usize {
        0
    }
}
