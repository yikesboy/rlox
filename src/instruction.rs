use std::fmt::{self, Display};

use crate::opcode::OpCode;

#[derive(Debug, Clone)]
pub enum Instruction {
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
