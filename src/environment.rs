use crate::error::*;
use crate::object::*;
use crate::token::*;

use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;
use std::ops::Deref;

#[derive(Debug)]
pub struct Environment {
    pub values: HashMap<String, Object>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: &str, value: Object) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get_at(&self, distance: usize, name: &str) -> Object {
        if distance == 0 {
            self.values.get(name).unwrap().clone()
        } else {
            //self.enclosing.as_ref().unwrap().borrow().get_at(distance - 1, name) // or just call it on self and not on self.enclosing
            self.get_at(distance - 1, name)
        }
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxResult> {
        if let Some(object) = self.values.get(&name.lexeme) {
            Ok(object.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow().get(name)
        } else {
            let err_msg = format!("GET: Undefined variable '{}'.", &name.lexeme);
            Err(LoxResult::runtime_error(name, &err_msg))
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), LoxResult> {
        if let Entry::Occupied(mut object) = self.values.entry(name.lexeme.clone()) {
            object.insert(value);
            Ok(())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(LoxResult::runtime_error(
                name,
                &format!("ASSIGN: Undefined variable '{}'.", &name.lexeme),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_from_enclosed_environment() {
        let outter_env = Rc::new(RefCell::new(Environment::new()));
        let token = Token::new(TokenType::Identifier, "foo".to_string(), None, 0);
        outter_env.borrow_mut().define("foo", Object::Num(10.0));

        let inner_env = Environment::new_with_enclosing(Rc::clone(&outter_env));
        assert!(inner_env.get(&token).is_ok());
        assert_eq!(inner_env.get(&token).unwrap(), Object::Num(10.0));
    }

    #[test]
    fn can_enclose_an_environment() {
        let env = Rc::new(RefCell::new(Environment::new()));
        let inner = Environment::new_with_enclosing(Rc::clone(&env));
        assert_eq!(
            inner.enclosing.unwrap().borrow().values,
            env.borrow().values
        );
    }

    #[test]
    fn can_assign_to_variable_in_enclosed_environment() {
        let outter_env = Rc::new(RefCell::new(Environment::new()));
        let token = Token::new(TokenType::Identifier, "foo".to_string(), None, 0);
        outter_env.borrow_mut().define("foo", Object::Num(10.0));

        let mut inner_env = Environment::new_with_enclosing(Rc::clone(&outter_env));
        assert!(inner_env.assign(&token, Object::Num(20.0)).is_ok());
        assert_eq!(inner_env.get(&token).unwrap(), Object::Num(20.0));
        assert_eq!(outter_env.borrow().get(&token).unwrap(), Object::Num(20.0));
    }
}
