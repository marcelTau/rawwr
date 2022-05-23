use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "\"{x}\""),
            Object::Bool(x) => write!(f, "{}", x.to_string()),
            Object::Nil => write!(f, "nil"),
        }
    }
}
