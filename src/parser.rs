use crate::chunk::Chunk;
use crate::opcode::{BinaryOp, OpCode, UnaryOp};
use crate::parser_rule::ParserRule;
use crate::precedence::Precedence;
use crate::scanner::Scanner;
use crate::token::{Token, TokenType};
use crate::value::Value;
use crate::vm::InterpreterError;

pub struct Parser {
    current: Option<Token>,
    previous: Option<Token>,
    scanner: Scanner,
    pub chunk: Chunk,
    in_panic_mode: bool,
}

impl Parser {
    pub fn new(scanner: Scanner) -> Self {
        Self {
            current: None,
            previous: None,
            scanner: scanner,
            chunk: Chunk::new(),
            in_panic_mode: false,
        }
    }

    pub fn advance(&mut self) -> Result<(), InterpreterError> {
        self.previous = self.current;

        loop {
            match self.scanner.scan_token() {
                Ok(token) => {
                    self.current = Some(token);
                    return Ok(());
                }
                Err(err) => return Err(InterpreterError::ScannerError(err)),
            }
        }
    }

    pub fn consume(&mut self, type_: TokenType) -> Result<(), InterpreterError> {
        if let Some(current) = self.current
            && current.type_ == type_
        {
            self.advance()?;
            return Ok(());
        }

        Err(InterpreterError::CompileError)
    }

    pub fn expression(&mut self) -> Result<(), InterpreterError> {
        self.parse_precendence(Precedence::Assignment)
    }

    pub fn number(&mut self) -> Result<(), InterpreterError> {
        let token = self.previous.ok_or(InterpreterError::CompileError)?;

        let value: f32 = self
            .scanner
            .lexeme(&token)
            .parse()
            .expect("should be a number");

        self.emit_constant(Value::Number(value), token.line);

        Ok(())
    }

    pub fn grouping(&mut self) -> Result<(), InterpreterError> {
        self.expression();
        self.consume(TokenType::RightParen)?;
        Ok(())
    }

    pub fn unary(&mut self) -> Result<(), InterpreterError> {
        let token = self.previous.ok_or(InterpreterError::CompileError)?;
        let operator_type = token.type_;

        self.parse_precendence(Precedence::Unary)?;

        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::Unary(UnaryOp::OpNegate)),
            _ => {}
        }

        Ok(())
    }

    pub fn binary(&mut self) -> Result<(), InterpreterError> {
        let token = self.previous.ok_or(InterpreterError::CompileError)?;
        let operator_type = token.type_;

        let parse_rule = ParserRule::get_rule(operator_type);

        self.parse_precendence(parse_rule.precedence.next_n_higher(1))?;

        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::Binary(BinaryOp::OpAdd)),
            TokenType::Minus => self.emit_byte(OpCode::Binary(BinaryOp::OpSubtract)),
            TokenType::Star => self.emit_byte(OpCode::Binary(BinaryOp::OpMultiply)),
            TokenType::Slash => self.emit_byte(OpCode::Binary(BinaryOp::OpDivide)),
            _ => panic!("should not be reachable"),
        }

        Ok(())
    }

    fn parse_precendence(&mut self, precendence: Precedence) -> Result<(), InterpreterError> {
        self.advance()?;

        let token: Token = self.previous.ok_or(InterpreterError::CompileError)?;

        let prefix = ParserRule::get_rule(token.type_)
            .prefix
            .ok_or(InterpreterError::CompileError)?;

        prefix(self)?;

        while let Some(current) = self.current {
            let rule = ParserRule::get_rule(current.type_);

            if precendence > rule.precedence {
                break;
            }

            self.advance()?;

            let token = self.previous.ok_or(InterpreterError::CompileError)?;
            let infix = ParserRule::get_rule(token.type_)
                .infix
                .ok_or(InterpreterError::CompileError)?;

            infix(self)?;
        }

        Ok(())
    }

    fn emit_constant(&mut self, value: Value, line: usize) {
        //self.emit_byte(OpCode::OpConstant);
        self.chunk.write_constant(value, line);
    }

    fn emit_byte(&mut self, op_code: OpCode) {
        if let Some(token) = self.previous {
            self.chunk.write(op_code, token.line);
        }
    }

    fn emit_bytes(&mut self, op_code1: OpCode, op_code2: OpCode) {
        self.emit_byte(op_code1);
        self.emit_byte(op_code2);
    }

    pub fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn);
    }
}
