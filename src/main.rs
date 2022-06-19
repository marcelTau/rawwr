#![allow(unused_variables, dead_code)]

//mod ast_printer;
mod environment;
mod error;
mod expr;
mod interpreter;
mod object;
mod parser;
mod scanner;
mod stmt;
mod token;
mod utils;
mod callable;
mod native_functions;
mod function;
mod resolver;
mod class;
mod instance;

use error::*;
use interpreter::*;
use parser::*;
use scanner::*;
use resolver::*;

use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::cmp::Ordering;
use std::rc::Rc;

struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    fn new() -> Self {
        Lox {
            interpreter: Interpreter::new(),
        }
    }
    fn run_file(&mut self, path: &str) -> io::Result<()> {
        let content = fs::read_to_string(path)?;

        if self.run(&content).is_err() {
            std::process::exit(1);
        }

        Ok(())
    }

    fn run_repl(&mut self) -> io::Result<()> {
        let mut line = String::new();
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut line).unwrap();
            let _ = self.run(&line);
            line.clear();
        }
    }

    fn run(&mut self, source_code: &str) -> Result<(), LoxResult> {
        let mut scanner = Scanner::new(source_code);
        let tokens = scanner.tokenize()?;

        let mut parser = Parser::new(tokens);
        let statements = parser.parse()?;


        if parser.success() {
            let resolver = Resolver::new(&self.interpreter);
            let s = Rc::new(statements);

            resolver.resolve(Rc::clone(&s))?;

            if resolver.success() {
                self.interpreter.interpret(Rc::clone(&s));
            }
        }
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut lox = Lox::new();

    match args.len().cmp(&2) {
        Ordering::Greater => {
            println!("USAGE: ./program [source_file]");
            std::process::exit(1);
        }
        Ordering::Less => {
            lox.run_repl()?;
        }
        Ordering::Equal => {
            lox.run_file(&args[1])?;
        }
    }
    Ok(())
}
