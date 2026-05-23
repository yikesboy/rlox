use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub enum OpCode {
    OpReturn,
    OpConstant,
}

impl OpCode {
    pub fn instruction_size(&self) -> usize {
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
