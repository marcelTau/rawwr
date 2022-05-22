#![allow(unused_variables, dead_code)]

use std::env;
use std::fs;
use std::io;
use std::io::Write;

mod scanner;
use scanner::*;

mod error;
use error::*;

//mod generate_ast;
//use generate_ast::generate_ast;

mod object;
mod parser;
use parser::*;
mod ast_printer;
use ast_printer::*;
mod token;
//use token::*;
mod utils;
mod expr;
//use expr::*;

fn main() -> std::io::Result<()> {
    //generate_ast("./src".to_string(), "Expr".to_string(), &vec![
      //"Binary   : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
      //"Grouping : Box<Expr> expression".to_string(),
      //"Literal  : Option<Object> value".to_string(),
      //"Unary    : Token operator, Box<Expr> right".to_string()
    //])?;

    //let expression = Expr::Binary (
        //BinaryExpr { 
            //left: Box::new(Expr::Unary(UnaryExpr {
                //operator: Token::new(TokenType::Minus, "-".to_string(), Literal::new(), 1),
                //right: Box::new(Expr::Literal(LiteralExpr {
                    //value: Literal::new_number(123 as f64),
                //}))
            //})),
            //operator: Token::new(TokenType::Star, "*".to_string(), Literal::new(), 1),
            //right: Box::new(Expr::Grouping(GroupingExpr {
                //expression: Box::new(Expr::Literal(LiteralExpr {
                    //value: Literal::new_number(45.67),
                //}))
            //})),
        //}
    //);

    //let p = AstPrinter;
    //println!("expression: {}", p.print(&expression).unwrap());


    //std::process::exit(0);
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("USAGE: ./program [source_file]");
        std::process::exit(1);
    } else if args.len() == 2 {
        run_file(&args[1])?;
    } else {
        run_repl()?;
    }

    Ok(())
}

fn run_file(path: &String) -> io::Result<()> {
    let content = fs::read_to_string(path)?;

    run(&content);

    Ok(())
}

fn run_repl() -> io::Result<()> {
    let mut line = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();
        run(&line);

        line.clear();
    }
}

fn run(source_code: &String) {
    let mut scanner = Scanner::new(source_code);
    let tokens = match scanner.tokenize() {
        Ok(tokens) => tokens,
        Err(err) => {
            ScannerError::report(&err);
            vec![]
        }
    };

    for t in &tokens {
        println!("token: {}", t);
    }

    let mut parser = Parser::new(tokens);
    let printer = AstPrinter {};

    match parser.parse() {
        None => (),
        Some(expr) => println!("{}", printer.print(&expr).unwrap()),
    }

}
