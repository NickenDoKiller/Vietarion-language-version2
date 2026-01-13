#!/bin/bash
echo "🛡️ KHÔI PHỤC LUẬT #[{}] VÀ #[ime] #[end] - KHÔNG ĐỂ MẤT MỘT CHỮ! 🛡️"

cat <<EOF > crates/vl_core/src/parser.rs
use crate::ast::{Stmt, Expr};
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct VlError { pub msg_vi: String, pub msg_en: String, pub line: usize, pub col: usize }

pub struct Parser { tokens: Vec<Token>, current: usize }

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self { Self { tokens, current: 0 } }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, VlError> {
        let mut stmts = vec![];
        while !self.is_at_end() {
            if let Some(stmt) = self.parse_stmt()? { stmts.push(stmt); }
        }
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Option<Stmt>, VlError> {
        // --- CHẶN ĐẦU HỨNG #[ ---
        if matches!(self.peek().kind, TokenType::SHARP) {
            self.handle_special_attribute()?;
            return self.parse_stmt(); // Sau khi xử lý xong # thì quay lại parse lệnh chính
        }

        match self.peek().kind {
            TokenType::TB => self.var_decl(),
            TokenType::NEU => self.if_stmt(),
            TokenType::LAP => self.while_stmt(),
            TokenType::EOF => { self.advance(); Ok(None) },
            _ => self.expression_stmt(),
        }
    }

    // Logic xử lý #[{}] và các loại #[ime], #[end]
    fn handle_special_attribute(&mut self) -> Result<(), VlError> {
        self.advance(); // Ăn dấu #
        
        if matches!(self.peek().kind, TokenType::LBRACKET) {
            self.advance(); // Ăn dấu [
            
            // Xử lý trường hợp #[{ ... }]
            if matches!(self.peek().kind, TokenType::LBRACE) {
                self.advance(); // Ăn dấu {
                while !matches!(self.peek().kind, TokenType::RBRACE) && !self.is_at_end() {
                    self.parse_stmt()?; // Parse nội dung bên trong khối attr
                }
                self.consume(TokenType::RBRACE, "Thieu } trong attribute block")?;
            } else {
                // Xử lý trường hợp #[ime], #[end]...
                while !matches!(self.peek().kind, TokenType::RBRACKET) && !self.is_at_end() {
                    self.advance(); 
                }
            }
            self.consume(TokenType::RBRACKET, "Thieu ] ket thuc attribute")?;
        }
        Ok(())
    }

    fn var_decl(&mut self) -> Result<Option<Stmt>, VlError> {
        self.advance(); // skip 'tb'
        let name = if let TokenType::TEN(ref n) = self.advance().kind { n.clone() } else { return Err(self.error("Thieu ten bien")); };
        if let TokenType::BANG = self.peek().kind { self.advance(); }
        let init = self.parse_expr()?;
        Ok(Some(Stmt::VarDecl { name, init }))
    }

    fn expression_stmt(&mut self) -> Result<Option<Stmt>, VlError> {
        let expr = self.parse_expr()?;
        Ok(Some(Stmt::Expression(expr)))
    }

    fn parse_expr(&mut self) -> Result<Expr, VlError> { self.equality() }

    fn equality(&mut self) -> Result<Expr, VlError> {
        let mut expr = self.comparison()?;
        while matches!(self.peek().kind, TokenType::BANG) {
            self.advance();
            if matches!(self.peek().kind, TokenType::BANG) { self.advance(); }
            let right = self.comparison()?;
            expr = Expr::Binary { left: Box::new(expr), op: TokenType::BANG, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, VlError> {
        let mut expr = self.term()?;
        while matches!(self.peek().kind, TokenType::GT | TokenType::LT) {
            let op = self.advance();
            let right = self.term()?;
            expr = Expr::Binary { left: Box::new(expr), op: op.kind, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, VlError> {
        let mut expr = self.call()?;
        while matches!(self.peek().kind, TokenType::PLUS | TokenType::MINUS) {
            let op = self.advance();
            let right = self.call()?;
            expr = Expr::Binary { left: Box::new(expr), op: op.kind, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn call(&mut self) -> Result<Expr, VlError> {
        let mut expr = self.primary()?;
        loop {
            if let TokenType::LPAREN = self.peek().kind {
                self.advance();
                let mut args = vec![];
                if !matches!(self.peek().kind, TokenType::RPAREN) {
                    loop {
                        args.push(self.parse_expr()?);
                        if let TokenType::COMMA = self.peek().kind { self.advance(); } else { break; }
                    }
                }
                self.consume(TokenType::RPAREN, "Thieu )")?;
                if let Expr::Variable(name) = expr {
                    expr = Expr::Call { callee: name, args };
                }
            } else { break; }
        }
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, VlError> {
        let token = self.advance();
        match token.kind {
            TokenType::Int(n) => Ok(Expr::Literal(TokenType::Int(n))),
            TokenType::CHUOI(s) => Ok(Expr::Literal(TokenType::CHUOI(s))),
            TokenType::TEN(n) => Ok(Expr::Variable(n)),
            TokenType::IN => Ok(Expr::Variable("in".into())),
            TokenType::TH => Ok(Expr::Variable("ngu".into())),
            TokenType::DOC_FILE => Ok(Expr::Variable("nhap".into())),
            TokenType::NGAUNHIEN => Ok(Expr::Variable("ngaunhien".into())),
            TokenType::LPAREN => {
                let expr = self.parse_expr()?;
                self.consume(TokenType::RPAREN, "Thieu )")?;
                Ok(expr)
            }
            _ => Err(VlError { msg_vi: format!("Loi tai: {:?}", token.lexeme), msg_en: "".into(), line: 0, col: 0 }),
        }
    }

    fn if_stmt(&mut self) -> Result<Option<Stmt>, VlError> {
        self.advance(); // neu
        let condition = self.parse_expr()?;
        self.consume(TokenType::LBRACE, "Thieu {")?;
        let mut then_branch = vec![];
        while !matches!(self.peek().kind, TokenType::RBRACE) && !self.is_at_end() {
            if let Some(s) = self.parse_stmt()? { then_branch.push(s); }
        }
        self.consume(TokenType::RBRACE, "Thieu }")?;
        Ok(Some(Stmt::If { condition, then_branch, else_branch: None }))
    }

    fn while_stmt(&mut self) -> Result<Option<Stmt>, VlError> {
        self.advance(); // lap
        let condition = self.parse_expr()?;
        self.consume(TokenType::LBRACE, "Thieu {")?;
        let mut body = vec![];
        while !matches!(self.peek().kind, TokenType::RBRACE) && !self.is_at_end() {
            if let Some(s) = self.parse_stmt()? { body.push(s); }
        }
        self.consume(TokenType::RBRACE, "Thieu }")?;
        Ok(Some(Stmt::While { condition, body }))
    }

    fn advance(&mut self) -> Token { if !self.is_at_end() { self.current += 1; } self.tokens[self.current - 1].clone() }
    fn peek(&self) -> Token { self.tokens[self.current].clone() }
    fn is_at_end(&self) -> bool { self.tokens[self.current].kind == TokenType::EOF }
    fn error(&self, msg: &str) -> VlError { VlError { msg_vi: msg.into(), msg_en: "".into(), line: 0, col: 0 } }
    fn consume(&mut self, kind: TokenType, msg: &str) -> Result<Token, VlError> {
        if self.peek().kind == kind { Ok(self.advance()) } else { Err(self.error(msg)) }
    }
}
EOF

echo "✅ ĐÃ KHÔI PHỤC LUẬT #[{}] VÀ #[ime]/#[end]! CHIẾN TIẾP THÔI! 🏎️💨"