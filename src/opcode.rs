
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    // Register Operations
    MOV = 0x90,    // MOV [to] [from] - store into register
    SUB = 0x92,    // SUB [registerOne] [registerTwo]
    ADD = 0x93,    // ADD [registerOne] [registerTwo]
    MUL = 0x94,    // MUL [registerOne] [registerTwo]
    DIV = 0x95,    // DIV [registerOne] [registerTwo]

    // Control Flow
    JMP = 0x80,    // JMP [address] - unconditional jump set RIP to address
    JIF = 0x81,    // JIF [address] - jump if register flag is false

    // Conditional Operations
    CMP = 0x82,    // CMP [registerOne] [registerTwo] - Check if equal, set flag
    LT = 0x83,     // LT [registerOne] [registerTwo] - Less than

    // I/O
    PRNT = 0xA0,   // PRNT [register] - prints value in register
    PRNTA = 0xA1,  // PRNTA [register] - prints address of value in register
}

#[derive(Debug, Copy, Clone)]
pub enum Args {
    Integers(i64),
    Strings(&'static str),
    Null(Option<i64>),
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub opcode: OpCode,
    pub arg_one: Args,
    pub arg_two: Args,
}
