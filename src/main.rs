use rvm::vm::VirtualMachine;
use rvm::opcode::{Instruction, OpCode, Args};

mod memory;
mod register;
mod opcode;

fn main() {
    let vm = VirtualMachine::new();
}