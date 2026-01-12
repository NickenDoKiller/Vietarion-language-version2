use crate::token::{Token, TokenType};
use crate::VlError;

pub struct Lexer {
    src: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            src: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, VlError> {
        self.skip_whitespace();
        if self.is_eof() { return Ok(self.make_token(TokenType::EOF, 0)); }

        let ch = self.peek();

        if ch.is_alphabetic() || ch == '_' { return Ok(self.read_identifier()); }
        if ch.is_ascii_digit() { return Ok(self.read_number()); }
        if ch == '"' { return self.read_string(); }

        // LOGIC XỬ LÝ KÝ TỰ GHÉP
        let token = match ch {
            '#' => {
                if self.peek_next() == '[' { self.read_mode_header()? } 
                else { self.skip_comment(); return self.next_token(); }
            }
            '=' => {
                if self.peek_next() == '=' { self.advance(); self.advance(); self.make_token(TokenType::Eq, 2) }
                else if self.peek_next() == '>' { self.advance(); self.advance(); self.make_token(TokenType::FatArrow, 2) }
                else { self.advance(); self.make_token(TokenType::Assign, 1) }
            }
            '-' => {
                if self.peek_next() == '>' { self.advance(); self.advance(); self.make_token(TokenType::Arrow, 2) }
                else { self.advance(); self.make_token(TokenType::Minus, 1) }
            }
            '<' => {
                if self.peek_next() == '=' { self.advance(); self.advance(); self.make_token(TokenType::LtEq, 2) }
                else { self.advance(); self.make_token(TokenType::Lt, 1) }
            }
            '>' => {
                if self.peek_next() == '=' { self.advance(); self.advance(); self.make_token(TokenType::GtEq, 2) }
                else { self.advance(); self.make_token(TokenType::Gt, 1) }
            }
            '!' => {
                if self.peek_next() == '=' { self.advance(); self.advance(); self.make_token(TokenType::NotEq, 2) }
                else { self.advance(); self.make_token(TokenType::Not, 1) }
            }
            // Ký tự đơn
            '+' => { self.advance(); self.make_token(TokenType::Plus, 1) }
            '*' => { self.advance(); self.make_token(TokenType::Star, 1) }
            '/' => { self.advance(); self.make_token(TokenType::Slash, 1) }
            '{' => { self.advance(); self.make_token(TokenType::LBrace, 1) }
            '}' => { self.advance(); self.make_token(TokenType::RBrace, 1) }
            '(' => { self.advance(); self.make_token(TokenType::LParen, 1) }
            ')' => { self.advance(); self.make_token(TokenType::RParen, 1) }
            ':' => { self.advance(); self.make_token(TokenType::Colon, 1) }
            ',' => { self.advance(); self.make_token(TokenType::Comma, 1) }
            '.' => { self.advance(); self.make_token(TokenType::Dot, 1) }
            _ => {
                let bad = self.advance();
                return Err(VlError {
                    msg_vi: format!("Ký tự gì lạ lùng vậy ba: '{}'", bad),
                    msg_en: format!("Unexpected character: '{}'", bad),
                    line: self.line, col: self.col - 1,
                });
            }
        };

        Ok(token)
    }

    fn read_identifier(&mut self) -> Token {
        let start_col = self.col;
        let mut res = String::new();
        while !self.is_eof() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            res.push(self.advance());
        }
        let kind = match res.as_str() {
            "tb" => TokenType::Tb, "cttd" => TokenType::Cttd, "neu" => TokenType::Neu,
            "neu_khong" => TokenType::NeuKhong, "ham" => TokenType::Ham, "tra_ve" => TokenType::TraVe,
            "xet" => TokenType::Xet, "khuon" => TokenType::Khuon, "hop" => TokenType::Hop,
            "trien_khai" => TokenType::TrienKhai, "ban_than" => TokenType::BanThan,
            "cong_khai" => TokenType::CongKhai, "dung" => TokenType::Dung, "sai" => TokenType::Sai,
            _ => TokenType::Ident(res.clone()),
        };
        Token { kind, line: self.line, col: start_col, length: res.len() }
    }

    fn read_number(&mut self) -> Token {
        let start_col = self.col;
        let mut res = String::new();
        while !self.is_eof() && self.peek().is_ascii_digit() { res.push(self.advance()); }
        Token { kind: TokenType::Int(res.parse().unwrap()), line: self.line, col: start_col, length: res.len() }
    }

    fn read_string(&mut self) -> Result<Token, VlError> {
        let start_col = self.col;
        self.advance(); // "
        let mut res = String::new();
        while !self.is_eof() && self.peek() != '"' { res.push(self.advance()); }
        if self.is_eof() { return Err(VlError { msg_vi: "Quên đóng ngoặc kép!".into(), msg_en: "Unterminated string".into(), line: self.line, col: start_col }); }
        self.advance(); // "
        let len = res.len() + 2;
        Ok(Token { kind: TokenType::Str(res), line: self.line, col: start_col, length: len })
    }

    fn read_mode_header(&mut self) -> Result<Token, VlError> {
        let start_col = self.col;
        let mut h = String::new();
        while !self.is_eof() && self.peek() != ']' { h.push(self.advance()); }
        if !self.is_eof() { h.push(self.advance()); }
        let kind = match h.as_str() {
            "#[{}]" => TokenType::ModeBrace, "#[ime]" => TokenType::ModeIndent,
            "#[end]" => TokenType::ModeEnd, _ => return Err(VlError { msg_vi: "Mode sai!".into(), msg_en: "Bad mode".into(), line: self.line, col: start_col }),
        };
        Ok(Token { kind, line: self.line, col: start_col, length: h.len() })
    }

    fn skip_whitespace(&mut self) {
        while !self.is_eof() && self.peek().is_whitespace() {
            if self.peek() == '\n' { self.line += 1; self.col = 0; }
            self.advance();
        }
    }
    fn skip_comment(&mut self) { while !self.is_eof() && self.peek() != '\n' { self.advance(); } }
    fn advance(&mut self) -> char { let c = self.src[self.pos]; self.pos += 1; self.col += 1; c }
    fn peek(&self) -> char { if self.is_eof() { '\0' } else { self.src[self.pos] } }
    fn peek_next(&self) -> char { if self.pos + 1 >= self.src.len() { '\0' } else { self.src[self.pos + 1] } }
    fn is_eof(&self) -> bool { self.pos >= self.src.len() }
    fn make_token(&self, kind: TokenType, len: usize) -> Token { Token { kind, line: self.line, col: self.col - len, length: len } }
}
