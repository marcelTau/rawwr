#![allow(unused_variables, dead_code)]

mod ast_printer;
mod error;
mod expr;
mod stmt;
mod interpreter;
mod object;
mod parser;
mod scanner;
mod token;
mod utils;

use error::*;
use interpreter::*;
use parser::*;
use scanner::*;

use std::env;
use std::fs;
use std::io;
use std::io::Write;

fn main() -> std::io::Result<()> {
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

    if run(&content).is_ok() {
        std::process::exit(1);
    }

    Ok(())
}

fn run_repl() -> io::Result<()> {
    let mut line = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();
        let _ = run(&line);
        line.clear();
    }
}

fn run(source_code: &String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source_code);
    let tokens = scanner.tokenize()?;

    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    let interpreter = Interpreter {};
    if interpreter.interpret(&statements) {
        Ok(())
    } else {
        Err(LoxError::scanner_error(0, "something went wrong, please go ahead and fix your code"))
    }
}
