use crate::ast::{Stmt, Expr};
use crate::token::TokenType;
use vl_vm::{Chunk, Value};

pub struct Compiler { chunk: Chunk }

impl Compiler {
    pub fn new() -> Self { Self { chunk: Chunk { code: vec![], constants: vec![], names: vec![] } } }
    
    pub fn compile(&mut self, stmts: Vec<Stmt>) -> Chunk {
        self.chunk.code.push(11); 
        let main_jump_patch = self.chunk.code.len();
        self.emit_u16(0);

        for stmt in &stmts {
            if let Stmt::Function { name, body, .. } = stmt {
                let start_addr = self.chunk.code.len();
                let n_idx = self.get_or_create_name(name.clone());
                let c_idx = self.chunk.constants.len();
                self.chunk.constants.push(Value::Number(start_addr as f64));
                self.chunk.code.push(1); self.chunk.code.push(c_idx as u8);
                self.chunk.code.push(6); self.chunk.code.push(n_idx as u8);
                for s in body { self.compile_stmt(s.clone()); }
                self.chunk.code.push(17); 
            }
        }

        let main_start = self.chunk.code.len();
        self.chunk.code[main_jump_patch] = (main_start >> 8) as u8;
        self.chunk.code[main_jump_patch+1] = (main_start & 0xFF) as u8;

        for stmt in stmts {
            if !matches!(stmt, Stmt::Function { .. }) { self.compile_stmt(stmt); }
        }
        self.chunk.code.push(0); 
        self.chunk.clone()
    }

    fn emit_u16(&mut self, val: usize) {
        self.chunk.code.push((val >> 8) as u8);
        self.chunk.code.push((val & 0xFF) as u8);
    }

    fn compile_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::VarDecl { name, init } => {
                self.compile_expr(init);
                let idx = self.get_or_create_name(name);
                self.chunk.code.push(6); self.chunk.code.push(idx as u8);
            }
            Stmt::If { condition, then_branch, .. } => {
                self.compile_expr(condition);
                self.chunk.code.push(10);
                let if_patch = self.chunk.code.len(); self.emit_u16(0);
                for s in then_branch { self.compile_stmt(s); }
                let end_if = self.chunk.code.len();
                self.chunk.code[if_patch] = (end_if >> 8) as u8;
                self.chunk.code[if_patch+1] = (end_if & 0xFF) as u8;
            }
            Stmt::While { condition, body } => {
                let start = self.chunk.code.len();
                self.compile_expr(condition);
                self.chunk.code.push(10); 
                let exit_patch = self.chunk.code.len(); self.emit_u16(0);
                for s in body { self.compile_stmt(s); }
                self.chunk.code.push(11); self.emit_u16(start);
                let end = self.chunk.code.len();
                self.chunk.code[exit_patch] = (end >> 8) as u8;
                self.chunk.code[exit_patch+1] = (end & 0xFF) as u8;
            }
            Stmt::Expression(expr) => { self.compile_expr(expr); }
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
            Expr::Literal(TokenType::CHUOI(s)) => {
                let idx = self.chunk.constants.len();
                self.chunk.constants.push(Value::Str(s));
                self.chunk.code.push(1); self.chunk.code.push(idx as u8);
            }
            Expr::Variable(name) => {
                let idx = self.get_or_create_name(name);
                self.chunk.code.push(7); self.chunk.code.push(idx as u8);
            }
            Expr::Call { callee, args } => {
                for arg in args { self.compile_expr(arg); }
                match callee.as_str() {
                    "in" => self.chunk.code.push(8),
                    "in_dong" => self.chunk.code.push(25),
                    "ngu" => self.chunk.code.push(21),
                    "xoa" => self.chunk.code.push(22),
                    "nhap" => self.chunk.code.push(23),
                    "doc_file" => self.chunk.code.push(20),
                    "ghi_file" => self.chunk.code.push(24),
                    "ngaunhien" => self.chunk.code.push(15),
                    _ => {
                        let idx = self.get_or_create_name(callee);
                        self.chunk.code.push(18); self.chunk.code.push(idx as u8);
                    }
                }
            }
            Expr::Binary { left, op, right } => {
                self.compile_expr(*left); self.compile_expr(*right);
                match op {
                    TokenType::PLUS => self.chunk.code.push(2),
                    TokenType::MINUS => self.chunk.code.push(3),
                    TokenType::GT => self.chunk.code.push(12),
                    TokenType::LT => self.chunk.code.push(13),
                    TokenType::BANG => {
                         // Xử lý gán biến kiểu cũ nếu cần
                    }
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
