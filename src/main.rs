use rvm::memory::Memory;
use rvm::register::Register;
use rvm::opcode::{Instruction, OpCode, Args};

mod memory;
mod register;
mod opcode;

fn main() {
    let mut memory = Memory::new();
    let mut register = Register::new();

    // Example usage
    register.mov("eax", 10);
    register.mov("ebx", 20);
    register.print("eax");
    register.print("ebx");

    let instruction = Instruction::new(OpCode::ADD, Args::Strings("eax"), argTwo: Args::Strings("ebx"))'
    
      
}
