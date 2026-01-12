#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Annotation
    AnnotationStart,   // #[
    AnnotationName(String),
    AnnotationEnd,     // ]
    

    // Keywords (tạm thời, sẽ mở rộng)
    Neu,
    In,

    // Identifiers & literals
    Identifier(String),
    Number(i64),
    StringLiteral(String),

    // Operators & symbols
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    Greater,
    Less,
    LParen, // (
    RParen, // )
    Comma,  // ,


    LBrace, // {
    RBrace, // }

    Newline,
    EOF,
}
