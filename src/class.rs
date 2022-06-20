use crate::callable::*;
use crate::error::*;
use crate::instance::Instance;
use crate::interpreter::*;
use crate::object::*;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub struct Class {
    name: String,
    methods: HashMap<String, Object>,
}

impl Class {
    pub fn new(name: String, methods: HashMap<String, Object>) -> Self {
        Self { name, methods }
    }

    pub fn instantiate(
        &self,
        interpreter: &Interpreter,
        arguments: Vec<Object>,
        klass: Rc<Class>,
    ) -> Result<Object, LoxResult> {
        Ok(Object::Instance(Rc::new(Instance::new(klass))))
    }

    pub fn find_method(&self, name: String) -> Option<Object> {
        self.methods.get(&name).cloned()
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let methods = self
            .methods
            .keys()
            .cloned()
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{} with methods: {{ {} }}", self.name, methods)
    }
}

impl LoxCallable for Class {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult> {
        unreachable!();
    }
    fn arity(&self) -> usize {
        0
    }
}
