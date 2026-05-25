mod chunk;
mod cli;
mod compiler;
mod instruction;
mod opcode;
mod scanner;
mod token;
mod value;
mod vm;

use crate::cli::handle_input;

fn main() -> Result<(), String> {
    handle_input()
}
