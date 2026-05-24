mod chunk;
mod instruction;
mod opcode;
mod value;
mod vm;

use chunk::Chunk;
use opcode::OpCode;
use vm::VM;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write_constant(1.2, 123);
    chunk.write(OpCode::Unary(opcode::UnaryOp::OpNegate), 123);
    chunk.write_constant(1.2, 123);
    chunk.write(OpCode::Binary(opcode::BinaryOp::OpAdd), 123);
    //chunk.write(OpCode::OpReturn, 123);

    chunk.write_constant(1.2, 123);
    chunk.write(OpCode::Unary(opcode::UnaryOp::OpNegate), 123);
    chunk.write_constant(1.2, 123);
    chunk.write(OpCode::Binary(opcode::BinaryOp::OpSubtract), 123);
    //chunk.write(OpCode::OpReturn, 123);

    chunk.write_constant(1.2, 123);
    chunk.write(OpCode::Unary(opcode::UnaryOp::OpNegate), 123);
    chunk.write_constant(1.2, 123);
    chunk.write(OpCode::Binary(opcode::BinaryOp::OpMultiply), 123);
    //chunk.write(OpCode::OpReturn, 123);

    chunk.write_constant(1.2, 123);
    chunk.write(OpCode::Unary(opcode::UnaryOp::OpNegate), 123);
    chunk.write_constant(1.2, 123);
    chunk.write(OpCode::Binary(opcode::BinaryOp::OpDivide), 123);
    chunk.write(OpCode::OpReturn, 123);

    //chunk.disassemble("test");

    let mut vm = VM::new(chunk);
    let _ = vm.interpret();
}
