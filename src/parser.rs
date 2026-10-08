// The intermediary program: turns tokens into the instruction array the vm executes.
// Labels are collected as we go and patched in once every instruction has an index.

use std::collections::HashMap;

use crate::opcode::{Args, Instruction, OpCode};
use crate::register::Register;
use crate::tokenizer::{Lexer, Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    instructions: Vec<Instruction>,
    labels: HashMap<String, usize>,
    pending: Vec<(usize, usize, String, usize)>, // instruction, arg slot, label, line
}

pub fn assemble(source: &str) -> Vec<Instruction> {
    Parser::new(Lexer::new(source).tokenize()).parse()
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            instructions: vec![],
            labels: HashMap::new(),
            pending: vec![],
        }
    }

    pub fn parse(mut self) -> Vec<Instruction> {
        loop {
            match self.peek().kind {
                TokenKind::Newline => self.current += 1,
                TokenKind::EOF => break,
                TokenKind::Ident => self.statement(),
                _ => {
                    let token = self.peek();
                    panic!("Expected an opcode or label on line {}, found '{}'", token.line, token.lexeme);
                }
            }
        }

        self.resolve_labels();
        self.instructions
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn next(&mut self) -> Token {
        let token = self.tokens[self.current].clone();
        self.current += 1;
        token
    }

    fn statement(&mut self) {
        let name = self.next();

        if self.peek().kind == TokenKind::Colon {
            self.current += 1;
            self.labels.insert(name.lexeme.to_ascii_lowercase(), self.instructions.len());
            return;
        }

        let opcode = OpCode::from_name(&name.lexeme)
            .unwrap_or_else(|| panic!("Unknown opcode on line {}: {}", name.line, name.lexeme));

        let index = self.instructions.len();
        let mut args = [Args::Null, Args::Null];

        for slot in 0..opcode.arg_count() {
            if slot > 0 && self.peek().kind == TokenKind::Comma {
                self.current += 1;
            }
            args[slot] = self.argument(opcode, index, slot);
        }

        match self.peek().kind {
            TokenKind::Newline | TokenKind::EOF => {}
            _ => {
                let token = self.peek();
                panic!("Unexpected '{}' after {} on line {}", token.lexeme, name.lexeme, token.line);
            }
        }

        let [arg_one, arg_two] = args;
        self.instructions.push(Instruction { opcode, arg_one, arg_two });
    }

    fn argument(&mut self, opcode: OpCode, index: usize, slot: usize) -> Args {
        let token = self.next();

        match token.kind {
            TokenKind::Star => {
                let register = self.next();
                if register.kind != TokenKind::Ident || !Register::is_register(&register.lexeme) {
                    panic!("Expected a register after * on line {}", register.line);
                }
                Args::Deref(register.lexeme.to_ascii_lowercase())
            }

            TokenKind::Minus => {
                let number = self.next();
                if number.kind != TokenKind::Number {
                    panic!("Expected a number after - on line {}", number.line);
                }
                Args::Integers(-parse_number(&number))
            }

            TokenKind::Number => {
                let value = parse_number(&token);
                if opcode.takes_address() {
                    Args::Address(value as usize)
                } else {
                    Args::Integers(value)
                }
            }

            TokenKind::Ident => {
                if Register::is_register(&token.lexeme) {
                    Args::Strings(token.lexeme.to_ascii_lowercase())
                } else {
                    // a label we have not seen yet, filled in by resolve_labels
                    self.pending.push((index, slot, token.lexeme.to_ascii_lowercase(), token.line));
                    Args::Null
                }
            }

            _ => panic!("Expected an argument on line {}, found '{}'", token.line, token.lexeme),
        }
    }

    fn resolve_labels(&mut self) {
        for (index, slot, label, line) in std::mem::take(&mut self.pending) {
            let target = *self
                .labels
                .get(&label)
                .unwrap_or_else(|| panic!("Unknown label on line {}: {}", line, label));

            if slot == 0 {
                self.instructions[index].arg_one = Args::Address(target);
            } else {
                self.instructions[index].arg_two = Args::Address(target);
            }
        }
    }
}

fn parse_number(token: &Token) -> i64 {
    let text = &token.lexeme;

    let parsed = if let Some(hex) = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        i64::from_str_radix(hex, 16)
    } else {
        text.parse::<i64>()
    };

    parsed.unwrap_or_else(|_| panic!("Invalid number on line {}: {}", token.line, text))
}
