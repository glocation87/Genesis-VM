// Essentially a pseudo cpu, immediate registers for storing and evaluationg data

pub struct Register {
    // Arithmetic Registers
    eax: i64,
    ebx: i64,   
    ecx: i64, // Result of any operation on eax and ecx will be stoeed here

    // Conditional Registers
    rax: i64,
    rbx: i64,
    rcx: i64, // Result of conditionals will be flagged here [ 0 - false, 1 - true]
}

impl Register {
    pub fn new() -> Self {
        Register {
            eax: 0,
            ebx: 0,
            ecx: 0,

            rax: 0,
            rbx: 0,
            rcx: 0,
        }
    }

    /*pub fn dereference(&self, register: i64) -> i64 {

    }*/

    pub fn mov(&mut self, to: &'static str, value: i64) {
        match to {
            "eax" => self.eax = value,
            "ebx" => self.ebx = value,
            "ecx" => panic!("Cannot use MOV operation on ecx register"),
            "rax" => self.rax = value,
            "rbx" => self.rbx = value,
            "rcx" => panic!("Cannot use MOV operation on rcx register"),
            _ => panic!("Invalid register name"),
        }
    }

    pub fn print(&self, register: &'static str) {
        match register {
            "eax" => println!("GVM EAX[ {} ]", self.eax),
            "ebx" => println!("GVM EBX[ {} ]", self.ebx),
            "ecx" => println!("GVM ECX[ {} ]", self.ecx),
            "rax" => println!("GVM RAX[ {} ]", self.rax),
            "rbx" => println!("GVM RBX[ {} ]", self.rbx),
            "rcx" => println!("GVM RCX[ {} ]", self.rcx),
            _ => panic!("Invalid register name"),
        }
    }
}       