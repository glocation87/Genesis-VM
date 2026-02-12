use rvm::vm::VirtualMachine;
use rvm::opcode::{Instruction, OpCode, Args};

mod memory;
mod register;
mod opcode;

fn main() {
    let mut vm = VirtualMachine::new();
    // Example usage
    /*register.mov("eax", 10);
    register.mov("ebx", 20);
    register.print("eax");
    register.print("ebx");*/

    vm.run();
}
