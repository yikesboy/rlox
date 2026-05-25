use crate::chunk::Chunk;
use crate::parser::Parser;
use crate::token::TokenType;
use crate::vm::InterpreterError;

pub struct Compiler {
    parser: Parser,
}

impl Compiler {
    pub fn new(parser: Parser) -> Self {
        Self { parser }
    }

    pub fn compile(mut self) -> Result<Chunk, InterpreterError> {
        self.parser.advance()?;
        self.parser.expression()?;
        self.parser.consume(TokenType::Eof)?;
        self.end_compiler();
        Ok(self.parser.chunk)
    }

    fn end_compiler(&mut self) {
        self.parser.emit_return();

        #[cfg(feature = "debug-print")]
        self.current_chunk.disassemble();
    }
}
