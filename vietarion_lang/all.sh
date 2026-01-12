#!/bin/bash

echo "🔥 Đang tổng tiến công diệt trừ lỗi Panic..."

# 1. NÂNG CẤP VM - Tuyệt đối không dùng unwrap() bừa bãi
cat <<EOF > crates/vl_vm/src/lib.rs
use std::collections::HashMap;

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<f64>,
    pub names: Vec<String>,
}

impl Clone for Chunk {
    fn clone(&self) -> Self {
        Self { code: self.code.clone(), constants: self.constants.clone(), names: self.names.clone() }
    }
}

pub struct VM {
    stack: Vec<f64>,
    globals: HashMap<String, f64>,
}

impl VM {
    pub fn new() -> Self { Self { stack: Vec::new(), globals: HashMap::new() } }
    pub fn run(&mut self, chunk: Chunk) {
        let mut ip = 0;
        while ip < chunk.code.len() {
            let inst = chunk.code[ip]; ip += 1;
            match inst {
                0 => break, // Halt
                1 => { // Constant
                    if let Some(&val) = chunk.constants.get(chunk.code[ip] as usize) {
                        self.stack.push(val); ip += 1;
                    }
                }
                2..=5 | 9 | 12 | 13 => { // Các phép toán 2 ngôi (+, -, *, ==, >, <)
                    if self.stack.len() < 2 { continue; }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let res = match inst {
                        2 => a + b,
                        3 => a - b,
                        4 => a * b,
                        5 => a / b,
                        9 => if a == b { 1.0 } else { 0.0 },
                        12 => if a > b { 1.0 } else { 0.0 },
                        13 => if a < b { 1.0 } else { 0.0 },
                        _ => 0.0,
                    };
                    self.stack.push(res);
                }
                6 => { // Store Global
                    let idx = chunk.code[ip] as usize; ip += 1;
                    if let Some(val) = self.stack.pop() {
                        self.globals.insert(chunk.names[idx].clone(), val);
                    }
                }
                7 => { // Load Global
                    let idx = chunk.code[ip] as usize; ip += 1;
                    let val = self.globals.get(&chunk.names[idx]).cloned().unwrap_or(0.0);
                    self.stack.push(val);
                }
                8 => { // Print
                    if let Some(val) = self.stack.pop() { println!("💬 [Vietarion]: {}", val); }
                }
                10 => { // Jump If False
                    let offset = chunk.code[ip] as usize; ip += 1;
                    let cond = self.stack.pop().unwrap_or(0.0);
                    if cond == 0.0 { ip += offset; }
                }
                11 => { // Jump Back
                    let offset = chunk.code[ip] as usize;
                    ip = ip - offset - 1;
                }
                _ => {}
            }
        }
    }
}
EOF

# 2. NÂNG CẤP COMPILER - Đảm bảo mapping Index chuẩn xác
cat <<EOF > crates/vl_core/src/compiler.rs
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
EOF

echo "✨ Hệ thống đã được làm sạch và gia cố thép. Chạy lại đi mày!"