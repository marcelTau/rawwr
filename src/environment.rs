use crate::error::*;
use crate::object::*;
use crate::token::*;
use std::collections::HashMap;

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
            let err_msg = format!("Undefined variable '{}'.", &name.lexeme);
            Err(LoxError::runtime_error(name, &err_msg))
        }
    }
}
