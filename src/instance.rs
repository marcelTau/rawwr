use crate::class::*;
use crate::error::*;
use crate::object::*;
use crate::token::*;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub struct Instance {
    klass: Rc<Class>,
    fields: RefCell<HashMap<String, Object>>,
}

impl Instance {
    pub fn new(klass: Rc<Class>) -> Self {
        Self {
            klass: Rc::clone(&klass),
            fields: RefCell::new(HashMap::new()),
        }
    }

    pub fn get(&self, name: &Token, this: &Rc<Instance>) -> Result<Object, LoxResult> {
        if let Entry::Occupied(o) = self.fields.borrow_mut().entry(name.lexeme.clone()) {
            Ok(o.get().clone())
        } else if let Some(method) = self.klass.find_method(name.lexeme.clone()) {
            if let Object::Func(func) = method {
                Ok(func.bind(&Object::Instance(Rc::clone(this))))
            } else {
                Err(LoxResult::runtime_error(name, "'this' is defined on a non-function"))
            }
        } else {
            Err(LoxResult::runtime_error(
                name,
                &format!("Undefined propertiry '{}'.", name.lexeme.clone()),
            ))
        }
    }

    pub fn set(&self, name: &Token, value: &Object) -> Result<(), LoxResult> {
        self.fields.borrow_mut().insert(name.lexeme.clone(), value.clone());
        Ok(())
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut fields = Vec::new();
        for (k, v) in self.fields.borrow().iter() {
            if let Object::Str(value) = v {
                fields.push(format!("{k} = \"{v}\""));
            } else {
                fields.push(format!("{k} = {v}"));
            }
        }
        write!(f, "{} with {{ {} }}", self.klass, fields.join(", "))
    }
}
