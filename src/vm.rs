use crate::memory::Memory;
use crate::register::Register;
use crate::opcode::{Instruction, OpCode, Args};




struct VirtualMachine {
    register: Register,
    memory: Memory,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            register: Register::new(),
            memory: Memory::new(),
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction.opcode {
            OpCode::MOV => {
                let to = instruction.argOne as Args;
                let value: Args = instruction.argTwo as Args;
                self.register.mov(to, value);
            }
            // Implement other opcodes similarly...
            _ => {}
        }
    }
}