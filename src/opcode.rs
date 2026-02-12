
pub enum OpCode {
    // Register Operations
    MOV = 0x01F, // MOV [to] [from]- store into register
    SUB = 0x02F, // SUB [registerOne] [registerTwo]
    ADD = 0x03F, // ADD [registerOne] [registerTwo]
    MUL = 0x04F, // MUL [registerOne] [registerTwo]
    DIV = 0x05F, // DIV [registerOne] [registerTwo]

    // Control Flow
    JMP = 0x06F, // JMP [address] - unconditional jump set RIP to address
    JIF = 0x07F, // JIF [address] - jump if register flag is false after conditional operation [CMP or LT]

    // Conditional Operations
    CMP = 0x08F, // CMP [registerOne] [registerTwo] - Check if both values are the same and sets a register flag to true
    LT = 0x09F, // LT [registerOne] [registerTwo] - Less than

    PRNT = 0x0FF, // PRNT [register] - prints value in register
    PRNTA = 0x0FFD, // PRNTA [register] - prints address of value stored in register

    HALT = 0x0FFF
}

#[derive(Debug, Clone)]
pub enum Args {
    Integers(i64),
    Strings(&'static str),
    Null(Option<i64>),
}

pub struct Instruction {
    pub opcode: OpCode,
    pub arg_one: Args,
    pub arg_two: Args,
}
