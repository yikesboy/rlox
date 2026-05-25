use std::fmt::Write;

use crate::instruction::Instruction;
use crate::opcode::OpCode;
use crate::value::Value;

pub struct Chunk {
    code: Vec<Instruction>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, op: OpCode, line: usize) {
        self.code.push(Instruction::Op(op));
        self.lines.push(line)
    }

    pub fn write_constant(&mut self, value: Value, line: usize) {
        let index = self.constants.len();
        self.constants.push(value);

        self.code.push(Instruction::Op(OpCode::OpConstant));
        self.code.push(Instruction::Operand(index));

        self.lines.push(line);
        self.lines.push(line);
    }

    pub fn access(&self, index: usize) -> Option<&Instruction> {
        self.code.get(index)
    }

    pub fn access_constant(&self, index: usize) -> Option<&Value> {
        self.constants.get(index)
    }

    pub fn disassemble(&self, name: &str) {
        let mut output = format!("=={}==\n", name);
        let mut offset = 0;

        while offset < self.code.len() {
            offset = self.disassemble_instruction(&mut output, offset);
        }

        println!("{output}");
    }

    pub fn disassemble_instruction(&self, output: &mut String, offset: usize) -> usize {
        let _ = write!(output, "{:04} ", offset);

        if let Some(instruction) = self.code.get(offset as usize) {
            match *instruction {
                Instruction::Op(OpCode::OpConstant) => {
                    let Instruction::Operand(index) =
                        self.code.get(offset + 1).expect("should have instruction")
                    else {
                        panic!("should have constant index next");
                    };

                    let value = self
                        .constants
                        .get(*index)
                        .expect("should hold constant at index");

                    let _ = writeln!(output, "{} {:4} '{}'", OpCode::OpConstant, index, value);

                    offset + OpCode::OpConstant.instruction_size()
                }

                Instruction::Op(OpCode::OpReturn) => {
                    let _ = writeln!(output, "{}", OpCode::OpReturn);
                    offset + OpCode::OpReturn.instruction_size()
                }

                Instruction::Op(OpCode::Unary(u_op)) => {
                    let _ = writeln!(output, "{}", OpCode::Unary(u_op));
                    offset + OpCode::Unary(u_op).instruction_size()
                }
                Instruction::Op(OpCode::Binary(b_op)) => {
                    let _ = writeln!(output, "{}", OpCode::Binary(b_op));
                    offset + OpCode::Binary(b_op).instruction_size()
                }

                Instruction::Operand(_) => {
                    panic!("unexepcted operant at offset {offset}")
                }
            }
        } else {
            offset
        }
    }
}
