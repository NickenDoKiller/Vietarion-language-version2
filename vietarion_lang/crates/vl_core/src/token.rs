#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    Tb, Neu, NeuKhong, Lap, Cttd, Ham, TraVe, 
    Xet, Khuon, Hop, TrienKhai, BanThan, CongKhai,
    Dung, Sai,

    // Symbols
    LBrace, RBrace, LParen, RParen, 
    Assign, Plus, Minus, Star, Slash,
    Eq, Gt, Lt, GtEq, LtEq, NotEq, Not,
    Comma, Dot, Colon, Arrow, FatArrow,

    // Literals & Others
    Ident(String),
    Int(i64),
    Str(String),
    EOF,
    ModeBrace, ModeIndent, ModeEnd,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub line: usize,
    pub col: usize,
    pub length: usize, // Thêm lại field này vì Lexer cần
}
