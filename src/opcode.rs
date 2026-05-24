use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    OpReturn,
    OpConstant,
    Unary(UnaryOp),
    Binary(BinaryOp),
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    OpNegate,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
}

impl OpCode {
    pub fn instruction_size(&self) -> usize {
        match self {
            OpCode::OpReturn | OpCode::Unary(_) | OpCode::Binary(_) => 1,
            OpCode::OpConstant => 2,
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::OpReturn => write!(f, "OpReturn"),
            OpCode::OpConstant => write!(f, "OpConstant"),
            OpCode::Unary(UnaryOp::OpNegate) => write!(f, "OpNegate"),
            OpCode::Binary(BinaryOp::OpAdd) => write!(f, "OpAdd"),
            OpCode::Binary(BinaryOp::OpSubtract) => write!(f, "OpSubtract"),
            OpCode::Binary(BinaryOp::OpMultiply) => write!(f, "OpMultiply"),
            OpCode::Binary(BinaryOp::OpDivide) => write!(f, "OpDivide"),
        }
    }
}
