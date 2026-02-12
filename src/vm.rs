use std::any::Any;

use crate::memory::Memory;
use crate::register::Register;
use crate::opcode::{Instruction, OpCode, Args};

pub struct VirtualMachine {
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
                let value: Args = instruction.arg_two; // we need a way to check the argument values
                match value {
                    Args::Integers(val) => {
                        let destination: Args = instruction.arg_one;
                        match destination {
                            Args::Strings(reg) => self.register.mov(reg, val), // move integer value into register
                            _ => panic!("Invalid destination for MOV operation"), // MOV can only move values into registers
                        }
                    },
                    // TO-DO: string suggests register name, resolve register value and move
                    Args::Strings(val) => panic!("Cannot move string value into register"), 
                    Args::Null(_) => panic!("Cannot move null value into register"),
                }
                
            }

            OpCode::PRNT => {
                let register: Args = instruction.arg_one;
                match register {
                    Args::Strings(reg) => self.register.print(reg), // print value in register
                    _ => panic!("Invalid argument for PRINT operation"), // PRINT can only print values from registers
                }
            }
            // Implement other opcodes similarly...
            _ => {}
        }
    }
}