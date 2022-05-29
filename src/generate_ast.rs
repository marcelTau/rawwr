use std::fs::File;
use std::io::Write;

struct Type {
    base_name: String,
    fields: String,
}

pub fn generate_ast(
    output_dir: String,
    base_name: String,
    types: &[String],
) -> Result<(), std::io::Error> {
    let path: String = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    let mut file = File::create(path)?;

    writeln!(file, "#![allow(unused_imports)]")?;
    writeln!(file, "use crate::token::*;")?;
    writeln!(file, "use crate::object::*;")?;
    writeln!(file, "use crate::error::*;")?;
    if base_name == "Stmt" {
        writeln!(file, "use crate::expr::*;")?;
    } else if base_name == "Expr" {
        writeln!(file, "use std::rc::Rc;")?;
    }

    let mut ttypes: Vec<Type> = vec![];

    for ttype in types {
        let sp = ttype.split_once(':').unwrap();
        ttypes.push(Type {
            base_name: sp.0.trim().to_string(),
            fields: sp.1.trim().to_string(),
        });
    }

    writeln!(file, "\npub enum {} {{", base_name)?;
    for t in &ttypes {
        writeln!(file, "    {}({}{}),", t.base_name, t.base_name, base_name)?;
    }
    writeln!(file, "}}\n")?;

    writeln!(file, "impl {} {{", base_name)?;
    writeln!(
        file,
        "    pub fn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, LoxError> {{",
        base_name
    )?;
    writeln!(file, "        match self {{")?;
    for t in &ttypes {
        writeln!(
            file,
            "            {}::{}(x) => x.accept(visitor),",
            base_name, t.base_name
        )?;
    }
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;

    for t in &ttypes {
        writeln!(file, "pub struct {}{} {{", t.base_name, base_name)?;
        for field in t.fields.split(',') {
            let (rust_type, name) = field.trim().split_once(' ').unwrap();
            writeln!(file, "    pub {}: {},", name, rust_type)?;
        }
        writeln!(file, "}}\n")?;
    }

    writeln!(file, "pub trait {}Visitor<T> {{", base_name)?;

    for t in &ttypes {
        writeln!(
            file,
            "    fn visit_{}_{}(&self, {}: &{}{}) -> Result<T, LoxError>;",
            t.base_name.to_lowercase(),
            base_name.to_lowercase(),
            base_name.to_lowercase(),
            t.base_name,
            base_name,
        )?;
    }
    writeln!(file, "}}\n")?;

    for t in &ttypes {
        writeln!(file, "impl {}{} {{", t.base_name, base_name)?;
        writeln!(
            file,
            "    pub fn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, LoxError> {{",
            base_name
        )?;
        writeln!(
            file,
            "        visitor.visit_{}_{}(self)",
            t.base_name.to_lowercase(),
            base_name.to_lowercase()
        )?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}\n")?;
    }

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    generate_ast("./src".to_string(), "Expr".to_string(), &vec![
        "Assign   : Token name, Box<Expr> value".to_string(),
        "Binary   : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
        "Call     : Rc<Expr> callee, Token paren, Vec<Expr> arguments".to_string(),
        "Grouping : Box<Expr> expression".to_string(),
        "Literal  : Option<Object> value".to_string(),
        "Logical  : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
        "Unary    : Token operator, Box<Expr> right".to_string(),
        "Variable : Token name".to_string(),
    ])?;

    generate_ast("./src".to_string(), "Stmt".to_string(), &vec![
        "Block          : Vec<Stmt> statements".to_string(),
        "Expression     : Expr expression".to_string(),
        "If             : Expr condition, Box<Stmt> then_branch, Option<Box<Stmt>> else_branch".to_string(),
        "Print          : Expr expression".to_string(),
        "Var            : Token name, Option<Expr> initializer".to_string(),
        "While          : Expr condition, Box<Stmt> body".to_string(),
    ])?;

    Ok(())
}
