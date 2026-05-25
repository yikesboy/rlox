use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

pub fn handle_input() -> Result<(), String> {
    let mut args = env::args();

    args.next();

    match (args.next(), args.next()) {
        (None, None) => repl(),
        (Some(path), None) => run_file(&path)?,
        _ => {
            eprintln!("Usage: rlox [path]");
            exit(64);
        }
    }

    Ok(())
}

fn repl() {
    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        print!("> ");
        io::stdout().flush().expect("failed to flush stdout");

        line.clear();

        match stdin.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let input = line.trim_end();
                if input == "exit" || input == "quit" {
                    break;
                }
                run(input);
            }
            Err(err) => {
                eprintln!("Error reading line: {err}");
                break;
            }
        }
    }
}

fn run_file(path: &str) -> Result<(), String> {
    let source =
        fs::read_to_string(path).map_err(|err| format!("Could not read file '{path}': {err}"))?;

    run(&source);

    Ok(())
}

fn run(source: &str) {}
