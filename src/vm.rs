use crate::{
    chunk::Chunk,
    instruction::Instruction,
    opcode::{BinaryOp, OpCode, UnaryOp},
    value::Value,
};

#[derive(Debug)]
pub enum InterpreterError {
    CompileError,
    RuntimeError,
}

type InterpreterResult = Result<(), InterpreterError>;

const DEBUG_TRACE_CONDITION: bool = false;

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self) -> InterpreterResult {
        self.run()
    }

    fn run(&mut self) -> InterpreterResult {
        loop {
            // TODO: REWORK
            if DEBUG_TRACE_CONDITION {
                println!("");
                for value in self.stack.iter() {
                    print!("[");
                    print!(" {value} ");
                    print!("]");
                }
                println!("\n");

                let mut output = String::new();
                let _ = self.chunk.disassemble_instruction(&mut output, self.ip);
                println!("{output}");
            }

            let instruction = *self
                .chunk
                .access(self.ip)
                .ok_or(InterpreterError::RuntimeError)?;

            match instruction {
                Instruction::Op(OpCode::OpReturn) => match self.stack.pop() {
                    Some(value) => {
                        println!("{value}");
                        return Ok(());
                    }
                    None => return Err(InterpreterError::RuntimeError),
                },
                Instruction::Op(OpCode::OpConstant) => {
                    let operand = self
                        .chunk
                        .access(self.ip + 1)
                        .ok_or(InterpreterError::RuntimeError)?;

                    let Instruction::Operand(index) = operand else {
                        return Err(InterpreterError::RuntimeError);
                    };

                    let constant = self
                        .chunk
                        .access_constant(*index)
                        .expect("should have constant at index");

                    self.stack.push(*constant);
                    self.ip += OpCode::OpConstant.instruction_size();
                }
                Instruction::Op(OpCode::Unary(u_op)) => {
                    self.unary_op(u_op)?;
                    self.ip += OpCode::Unary(u_op).instruction_size();
                }
                Instruction::Op(OpCode::Binary(b_op)) => {
                    self.binary_op(b_op)?;
                    self.ip += OpCode::Binary(b_op).instruction_size();
                }
                Instruction::Operand(_) => return Err(InterpreterError::RuntimeError),
            }
        }
    }

    fn unary_op(&mut self, opcode: UnaryOp) -> InterpreterResult {
        let value = self.stack.pop().ok_or(InterpreterError::RuntimeError)?;

        let result = match opcode {
            UnaryOp::OpNegate => -value,
        };

        self.stack.push(result);

        return Ok(());
    }

    fn binary_op(&mut self, opcode: BinaryOp) -> InterpreterResult {
        let b = self.stack.pop().ok_or(InterpreterError::RuntimeError)?;
        let a = self.stack.pop().ok_or(InterpreterError::RuntimeError)?;

        let result = match opcode {
            BinaryOp::OpAdd => a + b,
            BinaryOp::OpSubtract => a - b,
            BinaryOp::OpMultiply => a * b,
            BinaryOp::OpDivide => a / b,
        };

        self.stack.push(result);

        return Ok(());
    }
}
