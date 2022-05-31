use core::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use crate::callable::*;
use crate::error::*;
use crate::stmt::*;
use crate::interpreter::*;
use crate::object::*;
use crate::environment::*;
use crate::token::*;

pub struct Function {
    name: Token,
    params: Rc<Vec<Token>>,
    body: Rc<Vec<Stmt>>,
    closure: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(declaration: &FunctionStmt, closure: &Rc<RefCell<Environment>>) -> Self {
        Function {
            name: declaration.name.clone(),
            params: Rc::clone(&declaration.params),
            body: Rc::clone(&declaration.body),
            closure: Rc::clone(&closure),
        }
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

