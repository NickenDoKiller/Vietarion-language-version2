#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    TB, LAP, IN, NGAUNHIEN, NEU, NGUOC_LAI, TH, DOC_FILE,
    PLUS, MINUS, GT, LT, BANG,
    LPAREN, RPAREN, LBRACE, RBRACE, COMMA,
    TEN(String), CHUOI(String), Int(f64),
    EOF
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
}
