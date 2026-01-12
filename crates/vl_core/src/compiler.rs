use crate::ast::{Stmt, Expr};
use crate::token::TokenType;
use vl_vm::{Chunk, Value};

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
                let loop_start = self.chunk.code.len();
                self.compile_expr(condition);
                self.chunk.code.push(10);
                let exit_patch = self.chunk.code.len();
                self.chunk.code.push(0);
                for s in body { self.compile_stmt(s); }
                self.chunk.code.push(11);
                self.chunk.code.push(loop_start as u8);
                self.chunk.code[exit_patch] = self.chunk.code.len() as u8;
            }
            Stmt::Expression(Expr::Binary { left, op, right }) if op == TokenType::Assign => {
                if let Expr::Variable(name) = *left {
                    self.compile_expr(*right);
                    let idx = self.get_or_create_name(name);
                    self.chunk.code.push(6); self.chunk.code.push(idx as u8);
                }
            }
            Stmt::Expression(expr) => {
                self.compile_expr(expr);
                self.chunk.code.push(14);
            }
            _ => {}
        }
    }
    fn compile_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Literal(TokenType::Int(v)) => {
                let idx = self.chunk.constants.len();
                self.chunk.constants.push(Value::Number(v as f64));
                self.chunk.code.push(1); self.chunk.code.push(idx as u8);
            }
            Expr::Literal(TokenType::Str(s)) => {
                let idx = self.chunk.constants.len();
                self.chunk.constants.push(Value::Str(s));
                self.chunk.code.push(1); self.chunk.code.push(idx as u8);
            }
            Expr::Variable(name) => {
                let idx = self.get_or_create_name(name);
                self.chunk.code.push(7); self.chunk.code.push(idx as u8);
            }
            Expr::Call { callee, args } => {
                if callee == "in" {
                    for arg in args { self.compile_expr(arg); self.chunk.code.push(8); }
                } else if callee == "ngaunhien" {
                    self.chunk.code.push(15);
                }
            }
            Expr::Binary { left, op, right } => {
                self.compile_expr(*left);
                self.compile_expr(*right);
                match op {
                    TokenType::Plus => self.chunk.code.push(2),
                    TokenType::Minus => self.chunk.code.push(3),
                    TokenType::Gt => self.chunk.code.push(12),
                    TokenType::Lt => self.chunk.code.push(13),
                    _ => {}
                }
            }
            _ => {}
        }
    }
    fn get_or_create_name(&mut self, name: String) -> usize {
        if let Some(pos) = self.chunk.names.iter().position(|x| x == &name) { pos }
        else { self.chunk.names.push(name); self.chunk.names.len() - 1 }
    }
}
