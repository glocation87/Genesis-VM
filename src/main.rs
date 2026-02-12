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
    /*register.mov("eax", 10);
    register.mov("ebx", 20);
    register.print("eax");
    register.print("ebx");*/
    
    let instruction = Instruction::new(OpCode::MOV, Args::Strings("eax"), Args::Integers(7));    
    let instruction = Instruction::new(OpCode::MOV, Args::Strings("ebx"), Args::Strings("eax")); // this will copy the value 
}
