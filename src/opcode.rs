use std::{any::Any, ffi::c_void};

pub enum OpCode {
    // Register Operations
    MOV, // MOV [to] [from]- store into register
    SUB, // SUB [registerOne] [registerTwo]
    ADD, // ADD [registerOne] [registerTwo]
    MUL, // MUL [registerOne] [registerTwo]
    DIV, // DIV [registerOne] [registerTwo]

    // Control Flow
    JMP, // JMP [address] - unconditional jump set RIP to address
    JIF, // JIF [address] - jump if register flag is false after conditional operation [CMP or LT]

    // Conditional Operations
    CMP, // CMP [registerOne] [registerTwo] - Check if both values are the same and sets a register flag to true
    LT, // LT [registerOne] [registerTwo] - Less than

    PRNT, // PRNT [register] - prints value in register
    PRNTA, // PRNTA [register] - prints address of value stored in register
}

#[derive(Debug, Clone)]
pub enum Args {
    Integers(i64),
    Strings(String),
    Null(),
}

pub struct Instruction {
    pub opcode: OpCode,
    pub argOne: Args,
    pub argTwo: Args,
}

impl Instruction {      
    pub fn new(opcode: OpCode, argOne: Args, argTwo: Args) -> Self {
        return Instruction { opcode, argOne, argTwo };
    }
    
}