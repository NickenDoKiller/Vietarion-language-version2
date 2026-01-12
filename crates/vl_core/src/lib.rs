pub mod token;
pub mod lexer;
pub mod ast;
pub mod compiler;
pub mod parser;

#[derive(Debug, Clone)]
pub struct VlError {
    pub msg_vi: String,
    pub msg_en: String,
    pub line: usize,
    pub col: usize,
}
