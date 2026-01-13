use crate::token::{Token, TokenType};

pub struct Lexer { source: Vec<char>, current: usize }

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self { source: source.chars().collect(), current: 0 }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while !self.is_at_end() {
            let start = self.current;
            let c = self.advance();
            match c {
                '(' => tokens.push(self.make_token(TokenType::LPAREN, start)),
                ')' => tokens.push(self.make_token(TokenType::RPAREN, start)),
                '{' => tokens.push(self.make_token(TokenType::LBRACE, start)),
                '}' => tokens.push(self.make_token(TokenType::RBRACE, start)),
                ',' => tokens.push(self.make_token(TokenType::COMMA, start)),
                '=' => tokens.push(self.make_token(TokenType::BANG, start)),
                '+' => tokens.push(self.make_token(TokenType::PLUS, start)),
                '-' => tokens.push(self.make_token(TokenType::MINUS, start)),
                '>' => tokens.push(self.make_token(TokenType::GT, start)),
                '<' => tokens.push(self.make_token(TokenType::LT, start)),
                '"' => tokens.push(self.string(start)),
                ' ' | '\r' | '\t' | '\n' => {},
                _ => {
                    if c.is_digit(10) { tokens.push(self.number(start)); }
                    else if c.is_alphabetic() || c == '_' { tokens.push(self.identifier(start)); }
                }
            }
        }
        tokens.push(Token { kind: TokenType::EOF, lexeme: "".into() });
        tokens
    }

    fn string(&mut self, start: usize) -> Token {
        while self.peek() != '"' && !self.is_at_end() { self.advance(); }
        self.advance();
        let val: String = self.source[start + 1..self.current - 1].iter().collect();
        Token { kind: TokenType::CHUOI(val.clone()), lexeme: val }
    }

    fn number(&mut self, start: usize) -> Token {
        while self.peek().is_digit(10) || self.peek() == '.' { self.advance(); }
        let s: String = self.source[start..self.current].iter().collect();
        Token { kind: TokenType::Int(s.parse().unwrap_or(0.0)), lexeme: s }
    }

    fn identifier(&mut self, start: usize) -> Token {
        while self.peek().is_alphanumeric() || self.peek() == '_' { self.advance(); }
        let text: String = self.source[start..self.current].iter().collect();
        let kind = match text.as_str() {
            "tb" => TokenType::TB,
            "lap" => TokenType::LAP,
            "in" | "in_dong" => TokenType::IN,
            "neu" => TokenType::NEU,
            "nguoc_lai" => TokenType::NGUOC_LAI,
            "ngu" => TokenType::TH,
            "ngaunhien" => TokenType::NGAUNHIEN,
            "nhap" | "doc_file" => TokenType::DOC_FILE,
            _ => TokenType::TEN(text.clone()),
        };
        Token { kind, lexeme: text }
    }

    fn advance(&mut self) -> char { self.current += 1; self.source[self.current - 1] }
    fn peek(&self) -> char { if self.is_at_end() { '\0' } else { self.source[self.current] } }
    fn is_at_end(&self) -> bool { self.current >= self.source.len() }
    fn make_token(&self, kind: TokenType, start: usize) -> Token {
        let lexeme: String = self.source[start..self.current].iter().collect();
        Token { kind, lexeme }
    }
}
