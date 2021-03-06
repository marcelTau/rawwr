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
    superclass: Option<Rc<Class>>,
}

impl Class {
    pub fn new(name: String, superclass: Option<Rc<Class>>, methods:  HashMap<String, Object>) -> Self {
        Self { name, methods, superclass }
    }

    pub fn instantiate(
        &self,
        interpreter: &Interpreter,
        arguments: Vec<Object>,
        klass: Rc<Class>,
    ) -> Result<Object, LoxResult> {
        let instance = Object::Instance(Rc::new(Instance::new(klass)));
        if let Some(Object::Func(initializer)) = self.find_method("init".to_string()) {
            if let Object::Func(init) = initializer.bind(&instance) {
                init.call(interpreter, arguments, None)?;
            }
        };
        Ok(instance)
    }

    pub fn find_method(&self, name: String) -> Option<Object> {
        if let Some(method) = self.methods.get(&name) {
            Some(method.clone())
        } else if let Some(superclass) = &self.superclass {
            superclass.find_method(name)
        } else {
            None
        }
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
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>, klass: Option<Rc<Class>>) -> Result<Object, LoxResult> {
        self.instantiate(interpreter, arguments, klass.unwrap())
    }
    fn arity(&self) -> usize {
        if let Some(Object::Func(initializer)) = self.find_method("init".to_string()) {
            initializer.arity()
        } else {
            0
        }
    }
}
