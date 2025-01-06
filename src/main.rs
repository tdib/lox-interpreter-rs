mod expression;
mod scanner;
mod token;
mod util;

use scanner::Scanner;
use token::Token;

use std::io::{self, BufRead, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{env, fs, process};

static ERROR_FLAG: AtomicBool = AtomicBool::new(false);

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args = env::args().collect::<Vec<String>>();
    match args.len() {
        // Running the program standalone - open REPL
        1 => {
            if let Err(e) = run_repl() {
                eprintln!("Error while running REPL: {e}");
                process::exit(74);
            }
        }
        // Providing a file - run given file
        2 => {
            if let Err(e) = run_file(args.get(2).expect("Failed to get source code file name")) {
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

fn run_file(path: &str) -> io::Result<()> {
    let bytes = fs::read(path)?;
    let content = String::from_utf8_lossy(&bytes).to_string();
    run(content);
    Ok(())
}

// NOTE: This is called runPrompt in crafting interpreters
fn run_repl() -> io::Result<()> {
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
        run(trimmed_line);
        set_error_flag(false);
    }

    Ok(())
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens();

    tokens.iter().for_each(|token| println!("{:?}", token));
}

pub fn lox_error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, r#where: &str, message: &str) {
    eprintln!("[line: {}] Error {}: {}", line, r#where, message);
    set_error_flag(true);
}

fn set_error_flag(value: bool) {
    ERROR_FLAG.store(value, Ordering::SeqCst);
}

fn get_error_flag() -> bool {
    ERROR_FLAG.load(Ordering::SeqCst)
}
