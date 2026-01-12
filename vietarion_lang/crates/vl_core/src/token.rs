#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    TB, LAP, IN, NGAUNHIEN,
    Identifier(String), Str(String), Int(i64),
    Plus, Minus, Gt, Lt, Assign,
    LParen, RParen, LBrace, RBrace,
    EOF
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub line: usize,
}
