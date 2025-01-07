mod error;
mod expression;
mod interpreter;
mod parser;
mod scanner;
mod token;
mod util;

use error::{get_error_flag, set_error_flag};
use parser::Parser;
use scanner::Scanner;
use token::Token;

use interpreter::Interpreter;
use std::io::{self, BufRead, Write};
use std::{env, fs, process};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args = env::args().collect::<Vec<String>>();
    let interpreter = Interpreter::new();
    match args.len() {
        // Running the program standalone - open REPL
        1 => {
            if let Err(e) = run_repl(interpreter) {
                eprintln!("Error while running REPL: {e}");
                process::exit(74);
            }
        }
        // Providing a file - run given file
        2 => {
            if let Err(e) = run_file(
                interpreter,
                args.get(2).expect("Failed to get source code file name"),
            ) {
                eprintln!("Error: {e}");
                process::exit(74);
            }
        }
        // Something else, correct the user
        _ => {
            println!("Usage: jlox [script]");
            process::exit(64)
        }
    };
}

fn run_file(interpreter: Interpreter, path: &str) -> io::Result<()> {
    let bytes = fs::read(path)?;
    let content = String::from_utf8_lossy(&bytes).to_string();
    run(&interpreter, content);

    if error::get_error_flag() {
        process::exit(65)
    }
    if error::get_runtime_error_flag() {
        process::exit(70)
    }
    Ok(())
}

fn run_repl(interpreter: Interpreter) -> io::Result<()> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        print!("> ");
        // Flush to ensure prompt is displayed immediately
        io::stdout().flush()?;

        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;

        // Break out of loop if EOF is reached
        if bytes_read == 0 {
            break;
        }

        let trimmed_line = line.trim().to_string();
        run(&interpreter, trimmed_line);
        set_error_flag(false);
    }

    Ok(())
}

fn run(interpreter: &Interpreter, source: String) {
    let mut scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens();

    let mut parser = Parser::new(tokens);
    let expression = parser.parse();

    if get_error_flag() {
        return;
    }

    interpreter.interpret(expression.expect("Something went wrong"));
}
