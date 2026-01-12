// src/ast/stmt.rs
use super::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
    Neu {
        condition: Expr,
        body: Vec<Stmt>,
    },

    Call {
        name: String,
        args: Vec<Expr>,
    },
}
