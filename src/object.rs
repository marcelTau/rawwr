use core::fmt;
use std::rc::Rc;

use crate::callable::Callable;
use crate::instance::Instance;
use crate::class::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Num(f64),
    Str(String),
    Bool(bool),
    Func(Callable),
    Class(Rc<Class>),
    Instance(Rc<Instance>),
    Nil,
    ArithmeticError,
    DivByZeroError,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "{x}"),
            Object::Bool(x) => write!(f, "{}", x),
            Object::Func(_) => write!(f, "<func>"),
            Object::Class(c) => write!(f, "<Class {}>", c),
            Object::Instance(i) => write!(f, "<Instance {}>", i),
            Object::Nil => write!(f, "nil"),
            Object::ArithmeticError => write!(f, "ArithmeticError"),
            Object::DivByZeroError => write!(f, "DivByZeroError"),
        }
    }
}

impl std::ops::Mul for Object {
    type Output = Object;

    fn mul(self, other: Self) -> Object {
        match (self, other) {
            (Object::Num(left), Object::Num(right)) => Object::Num(left * right),
            _ => Object::ArithmeticError,
        }
    }
}

impl std::ops::Div for Object {
    type Output = Object;

    fn div(self, other: Self) -> Object {
        match (self, other) {
            (Object::Num(left), Object::Num(right)) => {
                if right == 0 as f64 {
                    Object::DivByZeroError
                } else {
                    Object::Num(left / right)
                }
            }
            _ => Object::ArithmeticError,
        }
    }
}

impl std::ops::Sub for Object {
    type Output = Object;

    fn sub(self, other: Self) -> Object {
        match (self, other) {
            (Object::Num(left), Object::Num(right)) => Object::Num(left - right),
            _ => Object::ArithmeticError,
        }
    }
}

impl std::ops::Add for Object {
    type Output = Object;

    fn add(self, other: Self) -> Object {
        match (self, other) {
            (Object::Num(left), Object::Num(right)) => Object::Num(left + right),
            (Object::Str(left), Object::Str(right)) => Object::Str(format!("{}{}", left, right)),
            _ => Object::ArithmeticError,
        }
    }
}

impl std::cmp::PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Object::Nil, Object::Nil) => Some(std::cmp::Ordering::Equal),
            (Object::Nil, _) | (_, Object::Nil) => None,
            (Object::Num(left), Object::Num(right)) => left.partial_cmp(right),
            _ => None,
        }
    }
}
