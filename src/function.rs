use core::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use crate::callable::*;
use crate::environment::*;
use crate::error::*;
use crate::interpreter::*;
use crate::object::*;
use crate::stmt::*;
use crate::token::*;

pub struct Function {
    name: Token,
    params: Rc<Vec<Token>>,
    body: Rc<Vec<Rc<Stmt>>>,
    closure: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(declaration: &FunctionStmt, closure: &Rc<RefCell<Environment>>) -> Self {
        Function {
            name: declaration.name.clone(),
            params: Rc::clone(&declaration.params),
            body: Rc::clone(&declaration.body),
            closure: Rc::clone(closure),
        }
    }

    pub fn bind(&self, instance: &Object) -> Object {
        let mut env = Environment::new_with_enclosing(Rc::clone(&self.closure));
        env.define("this", instance.clone());

        Object::Func(Rc::new(Self {
            name: self.name.clone(),
            params: Rc::clone(&self.params),
            body: Rc::clone(&self.body),
            closure: Rc::new(RefCell::new(env))
        }))
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            params: Rc::clone(&self.params),
            body: Rc::clone(&self.body),
            closure: Rc::clone(&self.closure),
        }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name.token_type == other.name.token_type
            && Rc::ptr_eq(&self.params, &other.params)
            && Rc::ptr_eq(&self.body, &other.body)
            && Rc::ptr_eq(&self.closure, &other.closure)
    }
}

impl LoxCallable for Function {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult> {
        let mut env = Environment::new_with_enclosing(Rc::clone(&self.closure));

        for (param, arg) in self.params.iter().zip(arguments.iter()) {
            env.define(param.lexeme.as_str(), arg.clone());
        }

        match interpreter.execute_block(&self.body, env) {
            Err(LoxResult::ReturnValue { value }) => Ok(value),
            Err(e) => Err(e),
            Ok(_) => Ok(Object::Nil),
        }
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<fn {}>", self.name.lexeme)
    }
}
