use crate::token::{Token, TokenType};
use crate::lexer::Lexer;
use crate::ast::{Expr, Stmt};
use crate::VlError;

pub struct Parser { lexer: Lexer, current: Token, peek: Token }

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Self, VlError> {
        let current = lexer.next_token()?;
        let peek = lexer.next_token()?;
        Ok(Self { lexer, current, peek })
    }
    fn advance(&mut self) -> Result<(), VlError> {
        self.current = self.peek.clone();
        self.peek = self.lexer.next_token()?;
        Ok(())
    }
    pub fn parse_program(&mut self) -> Result<Vec<Stmt>, VlError> {
        let mut stmts = Vec::new();
        while self.current.kind != TokenType::EOF {
            if let Some(stmt) = self.parse_statement()? { stmts.push(stmt); }
        }
        Ok(stmts)
    }
    fn parse_statement(&mut self) -> Result<Option<Stmt>, VlError> {
        match &self.current.kind {
            TokenType::Tb => Ok(Some(self.parse_var_decl()?)),
            TokenType::Neu => {
                self.advance()?;
                let condition = self.parse_expression()?;
                self.advance()?; // {
                let mut then_branch = Vec::new();
                while self.current.kind != TokenType::RBrace {
                    if let Some(s) = self.parse_statement()? { then_branch.push(s); }
                }
                self.advance()?; // }
                Ok(Some(Stmt::If { condition, then_branch, else_branch: None }))
            }
            TokenType::Ident(n) if n == "lap" => {
                self.advance()?;
                let condition = self.parse_expression()?;
                self.advance()?; // {
                let mut body = Vec::new();
                while self.current.kind != TokenType::RBrace {
                    if let Some(s) = self.parse_statement()? { body.push(s); }
                }
                self.advance()?; // }
                Ok(Some(Stmt::While { condition, body }))
            }
            TokenType::Ident(n) if n == "in" => {
                self.advance()?; self.advance()?;
                let expr = self.parse_expression()?;
                self.advance()?; // )
                Ok(Some(Stmt::Expression(Expr::Call { callee: "in".into(), args: vec![expr] })))
            }
            _ => { self.advance()?; Ok(None) }
        }
    }
    fn parse_var_decl(&mut self) -> Result<Stmt, VlError> {
        self.advance()?;
        let name = if let TokenType::Ident(n) = &self.current.kind { n.clone() } else { panic!("Cần tên biến"); };
        self.advance()?; self.advance()?; // name + '='
        let init = self.parse_expression()?;
        Ok(Stmt::VarDecl { name, init })
    }
    fn parse_expression(&mut self) -> Result<Expr, VlError> {
        let mut expr = self.parse_primary()?;
        while matches!(self.current.kind, TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Eq | TokenType::Gt | TokenType::Lt) {
            let op = self.current.kind.clone();
            self.advance()?;
            let right = self.parse_primary()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }
    fn parse_primary(&mut self) -> Result<Expr, VlError> {
        let res = match &self.current.kind {
            TokenType::Int(v) => Expr::Literal(TokenType::Int(*v)),
            TokenType::Ident(n) => Expr::Variable(n.clone()),
            _ => { self.advance()?; Expr::Literal(TokenType::Int(0)) },
        };
        self.advance()?;
        Ok(res)
    }
}
