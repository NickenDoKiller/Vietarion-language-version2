// src/ast/node.rs
use super::stmt::Stmt;

#[derive(Debug)]
pub struct Program {
    pub body: Vec<Stmt>,
}
