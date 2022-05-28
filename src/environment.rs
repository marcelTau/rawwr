use crate::error::*;
use crate::object::*;
use crate::token::*;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &String, value: Object) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxError> {
        if let Some(object) = self.values.get(&name.lexeme) {
            Ok(object.clone())
        } else {
            let err_msg = format!("GET: Undefined variable '{}'.", &name.lexeme);
            Err(LoxError::runtime_error(name, &err_msg))
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), LoxError> {
        if let Entry::Occupied(mut object) = self.values.entry(name.lexeme.clone()) {
            object.insert(value);
            Ok(())
        } else {
            Err(LoxError::runtime_error(name, &format!("ASSIGN: Undefined variable '{}'.", &name.lexeme)))
        }
    }
}
