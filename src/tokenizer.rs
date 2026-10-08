#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Ident,     // MOV, eax, ecx
    Number,    // 10, 20
    Comma,     // ,
    Star,      // * - dereference
    Colon,     // : - label definition
    Minus,     // -
    Newline,   // \n
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
}

pub struct Lexer {
    source: Vec<char>,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            source: input.chars().collect(),
            current: 0,
            line: 1,
            tokens: vec![],
        }
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            let c = self.advance();

            match c {
                ' ' | '\t' | '\r' => {}
                '\n' => {
                    self.add_token(TokenKind::Newline, "\n");
                    self.line += 1;
                }
                ',' => self.add_token(TokenKind::Comma, ","),
                '*' => self.add_token(TokenKind::Star, "*"),
                ':' => self.add_token(TokenKind::Colon, ":"),
                '-' => self.add_token(TokenKind::Minus, "-"),

                ';' | '#' => self.comment(),

                '0'..='9' => self.number(c),
                'a'..='z' | 'A'..='Z' | '_' => self.identifier(c),

                _ => panic!("Unexpected character on line {}: {}", self.line, c),
            }
        }

        self.add_token(TokenKind::EOF, "");

        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let ch = self.source[self.current];
        self.current += 1;
        ch
    }

    fn add_token(&mut self, kind: TokenKind, lexeme: &str) {
        self.tokens.push(Token {
            kind,
            lexeme: lexeme.to_string(),
            line: self.line,
        });
    }

    fn comment(&mut self) {
        while !self.is_at_end() && self.source[self.current] != '\n' {
            self.current += 1;
        }
    }

    fn number(&mut self, first: char) {
        let mut value = first.to_string();

        // hex literals like 0x40000
        if first == '0' && !self.is_at_end() && (self.source[self.current] == 'x' || self.source[self.current] == 'X') {
            value.push(self.advance());

            while !self.is_at_end() && self.source[self.current].is_ascii_hexdigit() {
                value.push(self.advance());
            }
        } else {
            while !self.is_at_end() && self.source[self.current].is_ascii_digit() {
                value.push(self.advance());
            }
        }

        self.add_token(TokenKind::Number, &value);
    }

    fn identifier(&mut self, first: char) {
        let mut value = first.to_string();

        while !self.is_at_end()
            && (self.source[self.current].is_alphanumeric()
                || self.source[self.current] == '_')
        {
            value.push(self.advance());
        }

        self.add_token(TokenKind::Ident, &value);
    }
}
