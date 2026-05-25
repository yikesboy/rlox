use crate::scanner::Scanner;
use crate::token::TokenType;
use crate::vm::InterpreterError;

pub fn compile(source: String) -> Result<(), InterpreterError> {
    let mut scanner = Scanner::new(source);
    let mut line: Option<usize> = None;

    loop {
        let token = match scanner.scan_token() {
            Ok(token) => token,
            // TODO:REWORK
            Err(error) => {
                eprintln!("[line {}] scanner error: {:?}", error.line, error.kind);
                break;
            }
        };

        if line != Some(token.line) {
            print!("{:04} ", token.line);
            line = Some(token.line);
        } else {
            print!("    | ");
        }

        println!("{:?} '{}'", token.type_, scanner.lexeme(&token));

        if token.type_ == TokenType::Eof {
            break;
        }
    }

    Ok(())
}
