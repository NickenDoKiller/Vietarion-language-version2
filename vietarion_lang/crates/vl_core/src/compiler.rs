use crate::ast::{Stmt, Expr};
use crate::token::TokenType;
use vl_vm::Chunk;

pub struct Compiler { chunk: Chunk }

impl Compiler {
    pub fn new() -> Self { Self { chunk: Chunk { code: vec![], constants: vec![], names: vec![] } } }
    pub fn compile(&mut self, stmts: Vec<Stmt>) -> Chunk {
        for stmt in stmts { self.compile_stmt(stmt); }
        self.chunk.code.push(0);
        self.chunk.clone()
    }
    fn compile_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::VarDecl { name, init } => {
                self.compile_expr(init);
                let idx = self.get_or_create_name(name);
                self.chunk.code.push(6); self.chunk.code.push(idx as u8);
            }
            Stmt::While { condition, body } => {
                let start = self.chunk.code.len();
                self.compile_expr(condition);
                self.chunk.code.push(10); // JumpIfFalse
                let exit_idx = self.chunk.code.len();
                self.chunk.code.push(0);
                for s in body { self.compile_stmt(s); }
                let offset = self.chunk.code.len() - start + 2;
                self.chunk.code.push(11); // JumpBack
                self.chunk.code.push(offset as u8);
                self.chunk.code[exit_idx] = (self.chunk.code.len() - exit_idx - 1) as u8;
            }
            Stmt::Expression(Expr::Call { callee, args }) if callee == "in" => {
                self.compile_expr(args[0].clone());
                self.chunk.code.push(8);
            }
            _ => {}
        }
    }
    fn get_or_create_name(&mut self, name: String) -> usize {
        if let Some(pos) = self.chunk.names.iter().position(|x| x == &name) { pos }
        else { self.chunk.names.push(name); self.chunk.names.len() - 1 }
    }
    fn compile_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Literal(TokenType::Int(v)) => {
                self.chunk.constants.push(v as f64);
                let idx = (self.chunk.constants.len() - 1) as u8;
                self.chunk.code.push(1); self.chunk.code.push(idx);
            }
            Expr::Variable(name) => {
                let idx = self.get_or_create_name(name);
                self.chunk.code.push(7); self.chunk.code.push(idx as u8);
            }
            Expr::Binary { left, op, right } => {
                self.compile_expr(*left); self.compile_expr(*right);
                match op {
                    TokenType::Plus => self.chunk.code.push(2),
                    TokenType::Minus => self.chunk.code.push(3),
                    TokenType::Star => self.chunk.code.push(4),
                    TokenType::Eq => self.chunk.code.push(9),
                    TokenType::Gt => self.chunk.code.push(12),
                    TokenType::Lt => self.chunk.code.push(13),
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
