use crate::vm::InterpreterError;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Number(f32),
    Bool(bool),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{n}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl Value {
    pub fn add(self, rhs: Self) -> Result<Self, InterpreterError> {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            _ => Err(InterpreterError::RuntimeError("operands must be numbers")),
        }
    }

    pub fn sub(self, rhs: Self) -> Result<Self, InterpreterError> {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            _ => Err(InterpreterError::RuntimeError("operands must be numbers")),
        }
    }

    pub fn div(self, rhs: Self) -> Result<Self, InterpreterError> {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
            _ => Err(InterpreterError::RuntimeError("operands must be numbers")),
        }
    }

    pub fn mul(self, rhs: Self) -> Result<Self, InterpreterError> {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            _ => Err(InterpreterError::RuntimeError("operands must be numbers")),
        }
    }

    pub fn neg(self) -> Result<Self, InterpreterError> {
        match self {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(InterpreterError::RuntimeError(
                "operand must be of type number",
            )),
        }
    }
}
