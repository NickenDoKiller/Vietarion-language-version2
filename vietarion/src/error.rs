#[derive(Debug)]
pub struct CompilerError {
    pub vn: String,
    pub en: String,
    pub line: usize,
    pub col: usize,
}

impl CompilerError {
    pub fn new(vn: &str, en: &str, line: usize, col: usize) -> Self {
        Self {
            vn: vn.to_string(),
            en: en.to_string(),
            line,
            col,
        }
    }
}
