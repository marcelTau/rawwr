#![allow(unused_variables, dead_code)]
use std::io::Write;
use std::env;
use std::fs;
use std::io;

mod scanner;
use scanner::{Scanner, ScannerError};

mod token;
mod utils;

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

    for token in tokens {
        println!("{}", token);
    }
}
