
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    // Register Operations
    MOV = 0x90,    // MOV [to] [from] - store into register
    LEA = 0x91,    // LEA [register] [address] - load effective address into register
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

    RET = 0xA3,    // RET - return from function, set RIP to value in RAX
    HALT = 0xFF,   // HALT - stop execution
}

impl OpCode {
    pub fn from_name(name: &str) -> Option<OpCode> {
        let table = [
            ("MOV", OpCode::MOV),
            ("LEA", OpCode::LEA),
            ("SUB", OpCode::SUB),
            ("ADD", OpCode::ADD),
            ("MUL", OpCode::MUL),
            ("DIV", OpCode::DIV),
            ("JMP", OpCode::JMP),
            ("JIF", OpCode::JIF),
            ("CMP", OpCode::CMP),
            ("LT", OpCode::LT),
            ("PRNT", OpCode::PRNT),
            ("PRNTA", OpCode::PRNTA),
            ("RET", OpCode::RET),
            ("HALT", OpCode::HALT),
        ];

        table
            .iter()
            .find(|(text, _)| text.eq_ignore_ascii_case(name))
            .map(|(_, opcode)| *opcode)
    }

    // How many arguments the opcode expects in source
    pub fn arg_count(&self) -> usize {
        match self {
            OpCode::RET | OpCode::HALT => 0,
            OpCode::JMP | OpCode::JIF | OpCode::PRNT | OpCode::PRNTA => 1,
            _ => 2,
        }
    }

    // True for opcodes whose argument is an instruction address
    pub fn takes_address(&self) -> bool {
        matches!(self, OpCode::JMP | OpCode::JIF)
    }
}

#[derive(Debug, Clone)]
pub enum Args {
    Integers(i64),
    Strings(String),  // register name
    Deref(String),    // *register - value at the address held by the register
    Address(usize),
    Null,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: OpCode,
    pub arg_one: Args,
    pub arg_two: Args,
}
