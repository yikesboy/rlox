mod chunk;
mod instruction;
mod opcode;
mod value;

use chunk::Chunk;
use opcode::OpCode;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write_constant(123.0, 123);
    chunk.write(OpCode::OpReturn, 123);
    chunk.disassemble("test");
}
