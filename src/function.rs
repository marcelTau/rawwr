use core::fmt;
use std::rc::Rc;

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
}

impl Function {
    pub fn new(declaration: &FunctionStmt) -> Self {
        Function {
            name: declaration.name.clone(),
            params: Rc::clone(&declaration.params),
            body: Rc::clone(&declaration.body),
        }
    }
}

impl LoxCallable for Function {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult> {
        let mut env = Environment::new_with_enclosing(Rc::clone(&interpreter.globals));

        self.params.len();

        for (param, arg) in self.params.iter().zip(arguments.iter()) {
            env.define(param.lexeme.as_str(), arg.clone());
        }
        interpreter.execute_block(&self.body, env)?;
        Ok(Object::Nil)
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

