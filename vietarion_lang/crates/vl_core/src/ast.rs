use crate::token::TokenType;

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(TokenType),
    Variable(String),
    Binary { left: Box<Expr>, op: TokenType, right: Box<Expr> },
    Call { callee: String, args: Vec<Expr> },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl { name: String, init: Expr },
    While { condition: Expr, body: Vec<Stmt> },
    If { condition: Expr, then_branch: Vec<Stmt>, else_branch: Option<Vec<Stmt>> },
    Function { name: String, params: Vec<String>, body: Vec<Stmt> },
    Expression(Expr),
}
