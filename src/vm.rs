use crate::memory::Memory;
use crate::register::Register;
use crate::opcode::{Instruction, OpCode, Args};

pub struct VirtualMachine {
    instructions: Vec<Instruction>,
    register: Register,
    memory: Memory,
    rip: usize,    // instruction pointer
    running: bool,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            register: Register::new(),
            memory: Memory::new(),

            rip: 0,
            instructions: vec![],

            running: true,
        }
    }

    pub fn load(&mut self, instructions: Vec<Instruction>) {
        self.instructions = instructions;
        self.rip = 0;
        self.running = true;
    }

    pub fn register(&self) -> &Register {
        &self.register
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    // Resolves an argument down to a value: a literal, a register, or memory behind a pointer
    fn value(&self, arg: &Args) -> i64 {
        match arg {
            Args::Integers(value) => *value,
            Args::Strings(register) => self.register.resolve_register_value(register),
            Args::Deref(register) => self.memory.read(self.address(register)),
            Args::Address(address) => *address as i64,
            Args::Null => panic!("Missing argument"),
        }
    }

    // A register holding an address rather than a value
    fn address(&self, register: &str) -> usize {
        let value = self.register.resolve_register_value(register);
        if value < 0 {
            panic!("Negative memory address in {}", register);
        }
        value as usize
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction.opcode {
            //Register Operations
            OpCode::MOV => {
                let value = self.value(&instruction.arg_two);
                match &instruction.arg_one {
                    Args::Strings(register) => self.register.mov(register, value),
                    Args::Deref(register) => {
                        let address = self.address(register);
                        self.memory.write(address, value);
                    }
                    _ => panic!("Invalid destination for MOV operation"), // MOV can only move values into registers or memory
                }
            }

            OpCode::LEA => {
                let address = self.value(&instruction.arg_two);
                match &instruction.arg_one {
                    Args::Strings(register) => self.register.mov(register, address),
                    _ => panic!("LEA can only load an address into a register"),
                }
            }

            //Arithmethics Operations
            OpCode::ADD | OpCode::SUB | OpCode::MUL | OpCode::DIV => {
                let left = self.value(&instruction.arg_one);
                let right = self.value(&instruction.arg_two);

                let result = match instruction.opcode {
                    OpCode::ADD => left.wrapping_add(right),
                    OpCode::SUB => left.wrapping_sub(right),
                    OpCode::MUL => left.wrapping_mul(right),
                    _ => {
                        if right == 0 {
                            panic!("Division by zero");
                        }
                        left.wrapping_div(right)
                    }
                };

                self.register.store("ecx", result); // store result in ecx register
            }

            //Conditional Operations
            OpCode::CMP => {
                let left = self.value(&instruction.arg_one);
                let right = self.value(&instruction.arg_two);
                self.register.store("rcx", if left == right { 1 } else { 0 });
            }

            OpCode::LT => {
                let left = self.value(&instruction.arg_one);
                let right = self.value(&instruction.arg_two);
                self.register.store("rcx", if left < right { 1 } else { 0 });
            }

            //Control Flow
            OpCode::JMP => {
                self.rip = self.jump_target(&instruction.arg_one);
            }

            OpCode::JIF => {
                if self.register.resolve_register_value("rcx") == 0 {
                    self.rip = self.jump_target(&instruction.arg_one);
                }
            }

            OpCode::RET => {
                let target = self.register.resolve_register_value("rax");
                self.rip = self.jump_target(&Args::Integers(target));
            }

            OpCode::HALT => self.running = false,

            //I/O
            OpCode::PRNT => match &instruction.arg_one {
                Args::Strings(register) => self.register.print(register),
                other => println!("GVM [ {} ]", self.value(other)),
            },

            OpCode::PRNTA => {
                let address = self.value(&instruction.arg_one);
                if address < 0 {
                    panic!("Negative memory address");
                }
                let address = address as usize;
                println!("GVM -> 0x{:X} [{}]", address, self.memory.section(address));
            }
        }
    }

    fn jump_target(&self, arg: &Args) -> usize {
        let target = self.value(arg);
        if target < 0 || target as usize > self.instructions.len() {
            panic!("Jump target out of bounds: {}", target);
        }
        target as usize
    }

    pub fn run(&mut self) {
        while self.running && self.rip < self.instructions.len() {
            let instruction = self.instructions[self.rip].clone();
            self.rip += 1; // Move to the next instruction, jumps overwrite this
            self.execute(instruction);
        }
    }
}
