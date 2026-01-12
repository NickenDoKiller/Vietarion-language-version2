use crate::token::{Token, TokenType};

pub struct Lexer { src: Vec<char>, pos: usize, line: usize }

impl Lexer {
    pub fn new(src: String) -> Self {
        Self { src: src.chars().collect(), pos: 0, line: 1 }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while !self.is_at_end() {
            let c = self.advance();
            match c {
                ' ' | '\r' | '\t' => {}
                '\n' => { self.line += 1; }
                '{' => tokens.push(Token { kind: TokenType::LBrace, line: self.line }),
                '}' => tokens.push(Token { kind: TokenType::RBrace, line: self.line }),
                '(' => tokens.push(Token { kind: TokenType::LParen, line: self.line }),
                ')' => tokens.push(Token { kind: TokenType::RParen, line: self.line }),
                '+' => tokens.push(Token { kind: TokenType::Plus, line: self.line }),
                '-' => tokens.push(Token { kind: TokenType::Minus, line: self.line }),
                '>' => tokens.push(Token { kind: TokenType::Gt, line: self.line }),
                '<' => tokens.push(Token { kind: TokenType::Lt, line: self.line }),
                '=' => tokens.push(Token { kind: TokenType::Assign, line: self.line }),
                '"' => {
                    let mut s = String::new();
                    while self.peek() != '"' && !self.is_at_end() { s.push(self.advance()); }
                    if !self.is_at_end() { self.advance(); }
                    tokens.push(Token { kind: TokenType::Str(s), line: self.line });
                }
                _ if c.is_digit(10) => {
                    let mut n = c.to_string();
                    while self.peek().is_digit(10) { n.push(self.advance()); }
                    tokens.push(Token { kind: TokenType::Int(n.parse().unwrap_or(0)), line: self.line });
                }
                _ if c.is_alphabetic() => {
                    let mut s = c.to_string();
                    while self.peek().is_alphanumeric() || self.peek() == '_' { s.push(self.advance()); }
                    let kind = match s.as_str() {
                        "tb" => TokenType::TB,
                        "lap" => TokenType::LAP,
                        "in" => TokenType::IN,
                        "ngaunhien" => TokenType::NGAUNHIEN,
                        _ => TokenType::Identifier(s),
                    };
                    tokens.push(Token { kind, line: self.line });
                }
                _ => {}
            }
        }
        tokens.push(Token { kind: TokenType::EOF, line: self.line });
        tokens
    }

    fn is_at_end(&self) -> bool { self.pos >= self.src.len() }
    fn advance(&mut self) -> char {
        let c = self.src[self.pos];
        self.pos += 1;
        c
    }
    fn peek(&self) -> char { if self.is_at_end() { '\0' } else { self.src[self.pos] } }
}
