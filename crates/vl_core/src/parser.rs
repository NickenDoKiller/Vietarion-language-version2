use crate::ast::{Stmt, Expr};
use crate::token::{Token, TokenType};

// Định nghĩa trực tiếp ở đây để khỏi import lỗi
#[derive(Debug)]
pub struct VlError {
    pub msg_vi: String,
    pub msg_en: String,
    pub line: usize,
    pub col: usize,
}

pub struct Parser {
    tokens: Vec<Token>,
    current_idx: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current_idx: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, VlError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            }
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Option<Stmt>, VlError> {
        let token = self.peek();
        match token.kind {
            TokenType::TB => self.parse_var_decl(),
            TokenType::LAP => self.parse_while(),
            TokenType::EOF => { self.advance(); Ok(None) },
            _ => self.parse_expression_stmt(),
        }
    }

    fn parse_var_decl(&mut self) -> Result<Option<Stmt>, VlError> {
        self.advance(); // skip 'tb'
        let name = if let TokenType::Identifier(ref n) = self.peek().kind { n.clone() } 
                   else { return Err(VlError { msg_vi: "Thiếu tên biến".into(), msg_en: "Missing var name".into(), line: 0, col: 0 }); };
        self.advance();
        
        if let TokenType::Assign = self.peek().kind {
            self.advance(); // skip '='
            let init = self.parse_expression()?;
            Ok(Some(Stmt::VarDecl { name, init }))
        } else {
            Ok(Some(Stmt::VarDecl { name, init: Expr::Literal(TokenType::Int(0)) }))
        }
    }

    fn parse_while(&mut self) -> Result<Option<Stmt>, VlError> {
        self.advance(); // skip 'lap'
        let condition = self.parse_expression()?;
        if let TokenType::LBrace = self.peek().kind { self.advance(); }
        let mut body = Vec::new();
        while !matches!(self.peek().kind, TokenType::RBrace | TokenType::EOF) {
            if let Some(s) = self.parse_statement()? { body.push(s); }
        }
        if let TokenType::RBrace = self.peek().kind { self.advance(); }
        Ok(Some(Stmt::While { condition, body }))
    }

    fn parse_expression_stmt(&mut self) -> Result<Option<Stmt>, VlError> {
        let expr = self.parse_expression()?;
        Ok(Some(Stmt::Expression(expr)))
    }

    fn parse_expression(&mut self) -> Result<Expr, VlError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, VlError> {
        let expr = self.parse_comparison()?;
        if let TokenType::Assign = self.peek().kind {
            self.advance();
            let value = self.parse_assignment()?;
            return Ok(Expr::Binary {
                left: Box::new(expr),
                op: TokenType::Assign,
                right: Box::new(value),
            });
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, VlError> {
        let mut expr = self.parse_sum()?;
        while matches!(self.peek().kind, TokenType::Gt | TokenType::Lt) {
            let op = self.advance().kind;
            let right = self.parse_sum()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn parse_sum(&mut self) -> Result<Expr, VlError> {
        let mut expr = self.parse_primary()?;
        while matches!(self.peek().kind, TokenType::Plus | TokenType::Minus) {
            let op = self.advance().kind;
            let right = self.parse_primary()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, VlError> {
        let token = self.advance();
        match token.kind {
            TokenType::Int(_) | TokenType::Str(_) => Ok(Expr::Literal(token.kind)),
            TokenType::Identifier(n) => {
                if let TokenType::LParen = self.peek().kind {
                    self.advance(); // skip (
                    let mut args = Vec::new();
                    while self.peek().kind != TokenType::RParen && !self.is_at_end() {
                        args.push(self.parse_expression()?);
                    }
                    if !self.is_at_end() { self.advance(); } // skip )
                    Ok(Expr::Call { callee: n, args })
                } else {
                    Ok(Expr::Variable(n))
                }
            },
            TokenType::IN | TokenType::NGAUNHIEN => {
                let name = if let TokenType::IN = token.kind { "in" } else { "ngaunhien" };
                if let TokenType::LParen = self.peek().kind {
                    self.advance();
                    let mut args = Vec::new();
                    while self.peek().kind != TokenType::RParen && !self.is_at_end() {
                        args.push(self.parse_expression()?);
                    }
                    if !self.is_at_end() { self.advance(); }
                    Ok(Expr::Call { callee: name.to_string(), args })
                } else {
                    Ok(Expr::Call { callee: name.to_string(), args: vec![] })
                }
            }
            _ => Err(VlError { msg_vi: format!("Lỗi tại dòng {}: {:?}", token.line, token.kind), msg_en: "Unexpected".into(), line: token.line, col: 0 }),
        }
    }

    fn advance(&mut self) -> Token {
        let t = self.tokens[self.current_idx].clone();
        if !self.is_at_end() { self.current_idx += 1; }
        t
    }
    fn peek(&self) -> &Token { &self.tokens[self.current_idx] }
    fn is_at_end(&self) -> bool { self.peek().kind == TokenType::EOF }
}
