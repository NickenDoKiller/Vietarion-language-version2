// src/ast/expr.rs
#[derive(Debug)]
pub enum Expr {
    Identifier(String),
    Number(i64),

    Compare {
        left: Box<Expr>,
        op: CompareOp,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum CompareOp {
    Greater,
    Less,
    Equal,
}
