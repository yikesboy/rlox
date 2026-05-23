use fmt::Display;
use std::fmt::{self, Write};

#[derive(Debug, Clone)]
enum OpCode {
    OpReturn,
    OpConstant,
}

impl OpCode {
    fn instruction_size(&self) -> usize {
        match self {
            OpCode::OpReturn => 1,
            OpCode::OpConstant => 2,
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::OpReturn => write!(f, "OpReturn"),
            OpCode::OpConstant => write!(f, "OpConstant"),
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Op(OpCode),
    Operand(usize),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Op(op) => write!(f, "{op}"),
            Instruction::Operand(index) => write!(f, "{index}"),
        }
    }
}

pub struct Chunk {
    code: Vec<Instruction>,
    constants: Vec<Value>,
    lines: Vec<i32>,
}

impl Chunk {
    fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    fn write(&mut self, op: OpCode, line: i32) {
        self.code.push(Instruction::Op(op));
        self.lines.push(line)
    }

    fn write_constant(&mut self, value: Value, line: i32) {
        let index = self.constants.len();
        self.constants.push(value);

        self.code.push(Instruction::Op(OpCode::OpConstant));
        self.code.push(Instruction::Operand(index));

        self.lines.push(line);
        self.lines.push(line);
    }

    fn disassemble(&self, name: &str) {
        let mut output = format!("=={}==\n", name);
        let mut offset = 0;

        while offset < self.code.len() {
            offset = self.disassemble_instruction(&mut output, offset);
        }

        println!("{output}");
    }

    fn disassemble_instruction(&self, output: &mut String, offset: usize) -> usize {
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

                    let _ = writeln!(output, "OpConstant {:4} '{}'", index, value);

                    offset + OpCode::OpConstant.instruction_size()
                }

                Instruction::Op(OpCode::OpReturn) => {
                    let _ = writeln!(output, "OpReturn");
                    offset + OpCode::OpReturn.instruction_size()
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

type Value = f32;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write_constant(123.0, 123);
    chunk.write(OpCode::OpReturn, 123);
    chunk.disassemble("test");
}
