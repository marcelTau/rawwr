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
    writeln!(file, "use std::rc::Rc;")?;
    writeln!(file, "use std::hash::{{Hash, Hasher}};")?;
    if base_name == "Stmt" {
        writeln!(file, "use crate::expr::*;")?;
    } else if base_name == "Expr" {
        // writeln!(file, "use std::hash::{{Hash, Hasher}};")?;
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
        //if base_name == "Expr" {
            writeln!(file, "    {}(Rc<{}{}>),", t.base_name, t.base_name, base_name)?;
        //} else {
            //writeln!(file, "    {}({}{}),", t.base_name, t.base_name, base_name)?;
        //}
    }
    writeln!(file, "}}\n")?;

    //if base_name == "Expr" {
    writeln!(file, "impl PartialEq for {} {{", base_name)?;
    writeln!(file, "    fn eq(&self, other: &Self) -> bool {{")?;
    writeln!(file, "        match (self, other) {{")?;

    for t in &ttypes {
        writeln!(file, "                  ({0}::{1}(a), {0}::{1}(b)) => Rc::ptr_eq(a, b),", base_name, t.base_name)?;
    }
    writeln!(file,"                  _ => false,")?;
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;


    writeln!(file, "\nimpl Eq for {}{{}}\n", base_name)?;
    writeln!(file, "impl Hash for {} {{", base_name)?;
    writeln!(file, "    fn hash<H>(&self, hasher: &mut H)")?;
    writeln!(file, "    where")?;
    writeln!(file, "        H: Hasher,")?;
    writeln!(file, "    {{")?;
    writeln!(file, "        match self {{")?;
    for t in &ttypes {
        writeln!(file, "        {base_name}::{}(a) => {{", t.base_name)?;
        writeln!(file, "            hasher.write_usize(Rc::as_ptr(a) as usize);")?;
        writeln!(file, "        }},")?;
    }
    writeln!(file, "    }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;
    //}


    writeln!(file, "impl {} {{", base_name)?;
    writeln!(
        file,
        "    pub fn accept<T>(&self, wrapper: Rc<{0}>, visitor: &dyn {0}Visitor<T>) -> Result<T, LoxResult> {{",
        base_name
    )?;
    writeln!(file, "        match self {{")?;
    for t in &ttypes {
        writeln!(
            file,
            "            {}::{}(x) => visitor.visit_{}_{}(wrapper, x),",
            base_name, t.base_name, t.base_name.to_lowercase(), base_name.to_lowercase()
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
            "    fn visit_{0}_{1}(&self, wrapper: Rc<{3}>, {1}: &{2}{3}) -> Result<T, LoxResult>;",
            t.base_name.to_lowercase(),
            base_name.to_lowercase(),
            t.base_name,
            base_name,
        )?;
    }
    writeln!(file, "}}\n")?;

    /*
    for t in &ttypes {
        writeln!(file, "impl {}{} {{", t.base_name, base_name)?;
        writeln!(
            file,
            "    pub fn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, LoxResult> {{",
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
        */
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    generate_ast("./src".to_string(), "Expr".to_string(), &vec![
        "Assign   : Token name, Rc<Expr> value".to_string(),
        "Binary   : Rc<Expr> left, Token operator, Rc<Expr> right".to_string(),
        "Call     : Rc<Expr> callee, Token paren, Vec<Rc<Expr>> arguments".to_string(),
        "Grouping : Rc<Expr> expression".to_string(),
        "Literal  : Option<Object> value".to_string(),
        "Logical  : Rc<Expr> left, Token operator, Rc<Expr> right".to_string(),
        "Unary    : Token operator, Rc<Expr> right".to_string(),
        "Variable : Token name".to_string(),
    ])?;

    generate_ast("./src".to_string(), "Stmt".to_string(), &vec![
        "Block          : Rc<Vec<Rc<Stmt>>> statements".to_string(),
        "Expression     : Rc<Expr> expression".to_string(),
        "Function       : Token name, Rc<Vec<Token>> params, Rc<Vec<Rc<Stmt>>> body".to_string(),
        "If             : Rc<Expr> condition, Rc<Stmt> then_branch, Option<Rc<Stmt>> else_branch".to_string(),
        "Print          : Rc<Expr> expression".to_string(),
        "Return         : Token keyword, Option<Rc<Expr>> value".to_string(),
        "Var            : Token name, Option<Rc<Expr>> initializer".to_string(),
        "While          : Rc<Expr> condition, Rc<Stmt> body".to_string(),
    ])?;

    Ok(())
}
