use crate::token::Token;
use crate::error::CompilerError;

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

    fn peek(&self) -> Option<char> {
        self.src.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.src.get(self.pos).copied();
        if let Some(c) = ch {
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        ch
    }

    pub fn next_token(&mut self) -> Result<Token, CompilerError> {
        while let Some(ch) = self.peek() {
            return match ch {
                ' ' | '\t' => {
                    self.advance();
                    continue;
                }
                '\n' => {
                    self.advance();
                    Ok(Token::Newline)
                }
                '#' => self.lex_annotation(),
                ']' => {
                    self.advance();
                    Ok(Token::AnnotationEnd)
                }
                '{' => {
                    self.advance();
                    Ok(Token::LBrace)
                }
                '}' => {
                    self.advance();
                    Ok(Token::RBrace)
                }
                '+' => {
                    self.advance();
                    Ok(Token::Plus)
                }
                '=' => {
                    self.advance();
                    Ok(Token::Equal)
                }
                '>' => {
                    self.advance();
                    Ok(Token::Greater)
                }
                '<' => {
                    self.advance();
                    Ok(Token::Less)
                }
                '(' => {
                    self.advance();
                    Ok(Token::LParen)
                }
                ')' => {
                    self.advance();
                    Ok(Token::RParen)
                }
                ',' => {
                    self.advance();
                    Ok(Token::Comma)
                }

                '"' => self.lex_string(),
                c if c.is_ascii_digit() => self.lex_number(),
                c if c.is_alphabetic() || c == '_' => self.lex_identifier(),
                _ => Err(CompilerError::new(
                    "Ký tự không hợp lệ",
                    "Invalid character",
                    self.line,
                    self.col,
                )),
            };
        }
        Ok(Token::EOF)
    }

    fn lex_annotation(&mut self) -> Result<Token, CompilerError> {
        self.advance(); // #
        if self.peek() != Some('[') {
            return Err(CompilerError::new(
                "Annotation phải bắt đầu bằng #[",
                "Annotation must start with #[",
                self.line,
                self.col,
            ));
        }
        self.advance(); // [
        Ok(Token::AnnotationStart)
    }

    fn lex_string(&mut self) -> Result<Token, CompilerError> {
        self.advance(); // "
        let mut value = String::new();
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance();
                return Ok(Token::StringLiteral(value));
            }
            value.push(ch);
            self.advance();
        }
        Err(CompilerError::new(
            "Chuỗi chưa được đóng",
            "Unterminated string literal",
            self.line,
            self.col,
        ))
    }

    fn lex_number(&mut self) -> Result<Token, CompilerError> {
        let mut num = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                num.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        Ok(Token::Number(num.parse().unwrap()))
    }

    fn lex_identifier(&mut self) -> Result<Token, CompilerError> {
        let mut ident = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let token = match ident.as_str() {
            "neu" => Token::Neu,
            "in" => Token::In,
            _ => Token::Identifier(ident),
        };
        Ok(token)
    }
}
