use crate::token::Token;
use crate::error::CompilerError;
use crate::ast::node::Program;
use crate::ast::stmt::Stmt;
use crate::ast::expr::{Expr, CompareOp};
use super::block::BlockMode;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,

    block: BlockMode,
    curly_depth: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            block: BlockMode::None,
            curly_depth: 0,
        }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    // ================= ENTRY =================

    pub fn parse(&mut self) -> Result<Program, CompilerError> {
        let mut body = Vec::new();

        while let Some(tok) = self.current() {
            match tok {
                Token::AnnotationStart => self.parse_annotation()?,
                Token::Newline => self.advance(),
                Token::Neu | Token::In => {
                    let stmt = self.parse_stmt()?;
                    body.push(stmt);
                }
                Token::EOF => break,
                _ => return Err(err("Statement không hợp lệ")),
            }
        }

        if self.block != BlockMode::None {
            return Err(err("Chưa đóng lãnh địa block"));
        }

        Ok(Program { body })
    }

    // ================= STATEMENT =================

    fn parse_stmt(&mut self) -> Result<Stmt, CompilerError> {
        match self.current() {
            Some(Token::Neu) => self.parse_neu_stmt(),
            Some(Token::In) => self.parse_call_stmt(),
            Some(Token::Newline) => {
                self.advance();
                self.parse_stmt()
            }
            _ => Err(err("Statement không hợp lệ")),
        }
    }

    // ================= CALL =================

    fn parse_call_stmt(&mut self) -> Result<Stmt, CompilerError> {
        // in
        self.advance();

        match self.current() {
            Some(Token::LParen) => self.advance(),
            _ => return Err(err("Thiếu ( sau lời gọi hàm")),
        }

        let mut args = Vec::new();

        match self.current() {
            Some(Token::StringLiteral(s)) => {
                args.push(Expr::Identifier(s.clone()));
                self.advance();
            }
            Some(Token::Identifier(s)) => {
                args.push(Expr::Identifier(s.clone()));
                self.advance();
            }
            _ => return Err(err("Thiếu tham số cho hàm")),
        }

        match self.current() {
            Some(Token::RParen) => self.advance(),
            _ => return Err(err("Thiếu ) sau lời gọi hàm")),
        }

        Ok(Stmt::Call {
            name: "in".to_string(),
            args,
        })
    }

    // ================= BLOCK =================

    fn on_lbrace(&mut self) -> Result<(), CompilerError> {
        if self.block != BlockMode::Curly {
            return Err(err("Không được dùng { khi chưa mở curly_brakets"));
        }
        self.curly_depth += 1;
        self.advance();
        Ok(())
    }

    fn on_rbrace(&mut self) -> Result<(), CompilerError> {
        if self.block != BlockMode::Curly {
            return Err(err("Không được dùng } khi chưa mở curly_brakets"));
        }
        if self.curly_depth == 0 {
            return Err(err("Dư dấu }"));
        }
        self.curly_depth -= 1;
        self.advance();
        Ok(())
    }

    // ================= ANNOTATION =================

    fn parse_annotation(&mut self) -> Result<(), CompilerError> {
        self.advance(); // #[

        let name = match self.current() {
            Some(Token::Identifier(s)) => s.clone(),
            _ => return Err(err("Annotation không hợp lệ")),
        };
        self.advance();

        match self.current() {
            Some(Token::AnnotationEnd) => self.advance(),
            _ => return Err(err("Thiếu dấu ]")),
        }

        match name.as_str() {
            "curly_brakets" | "{}" => {
                if self.block != BlockMode::None {
                    return Err(err("Đã ở trong block khác"));
                }
                self.block = BlockMode::Curly;
            }
            "curly_brakets_end" | "{}_end" => {
                if self.block != BlockMode::Curly {
                    return Err(err("Không có curly_brakets để đóng"));
                }
                if self.curly_depth != 0 {
                    return Err(err("Chưa đóng hết dấu {"));
                }
                self.block = BlockMode::None;
            }
            _ => return Err(err("Annotation không hỗ trợ")),
        }

        Ok(())
    }

    // ================= NEU =================

    fn parse_neu_stmt(&mut self) -> Result<Stmt, CompilerError> {
        self.advance(); // neu

        let left = match self.current() {
            Some(Token::Identifier(s)) => {
                let e = Expr::Identifier(s.clone());
                self.advance();
                e
            }
            _ => return Err(err("Thiếu biến sau neu")),
        };

        let op = match self.current() {
            Some(Token::Greater) => CompareOp::Greater,
            Some(Token::Less) => CompareOp::Less,
            Some(Token::Equal) => CompareOp::Equal,
            _ => return Err(err("Thiếu toán tử so sánh")),
        };
        self.advance();

        let right = match self.current() {
            Some(Token::Number(n)) => {
                let e = Expr::Number(*n);
                self.advance();
                e
            }
            Some(Token::Identifier(s)) => {
                let e = Expr::Identifier(s.clone());
                self.advance();
                e
            }
            _ => return Err(err("Thiếu giá trị so sánh")),
        };

        let condition = Expr::Compare {
            left: Box::new(left),
            op,
            right: Box::new(right),
        };

        match self.current() {
            Some(Token::LBrace) => self.on_lbrace()?,
            _ => return Err(err("Thiếu { sau neu")),
        }

        let mut body = Vec::new();

        while let Some(tok) = self.current() {
            match tok {
                Token::RBrace => {
                    self.on_rbrace()?;
                    break;
                }
                Token::Newline => self.advance(),
                _ => {
                    let stmt = self.parse_stmt()?;
                    body.push(stmt);
                }
            }
        }

        Ok(Stmt::Neu { condition, body })
    }
}

// ================= ERROR =================

fn err(vn: &str) -> CompilerError {
    CompilerError::new(vn, vn, 0, 0)
}
