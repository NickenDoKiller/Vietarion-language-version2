use std::collections::HashMap;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::io::{self, Write};
use std::fs;

#[derive(Debug, Clone)]
pub enum Value { Number(f64), Str(String), Nil }

impl Value {
    pub fn as_number(&self) -> f64 { 
        match self { Value::Number(n) => *n, Value::Str(s) => s.parse().unwrap_or(0.0), _ => 0.0 } 
    }
    pub fn is_truthy(&self) -> bool { 
        match self { Value::Number(n) => *n > 0.0, Value::Str(s) => !s.is_empty(), _ => false } 
    }
}

#[derive(Debug, Clone)]
pub struct Chunk { pub code: Vec<u8>, pub constants: Vec<Value>, pub names: Vec<String> }

pub struct VM {
    pub stack: [Value; 256],
    pub stack_top: usize,
    pub globals: HashMap<String, Value>,
    pub call_stack: Vec<usize>,
}

impl VM {
    pub fn new() -> Self {
        const NIL: Value = Value::Nil;
        Self { stack: [NIL; 256], stack_top: 0, globals: HashMap::new(), call_stack: vec![] }
    }
    fn push(&mut self, val: Value) { if self.stack_top < 256 { self.stack[self.stack_top] = val; self.stack_top += 1; } }
    fn pop(&mut self) -> Value { if self.stack_top == 0 { return Value::Nil; } self.stack_top -= 1; std::mem::replace(&mut self.stack[self.stack_top], Value::Nil) }
    fn read_u16(&self, code: &[u8], ip: &mut usize) -> usize { let res = ((code[*ip] as u16) << 8 | (code[*ip + 1] as u16)) as usize; *ip += 2; res }

    pub fn run(&mut self, chunk: Chunk) {
        let mut ip = 0;
        loop {
            if ip >= chunk.code.len() { break; }
            let opcode = chunk.code[ip]; ip += 1;
            match opcode {
                0 => break,
                1 => { let idx = chunk.code[ip] as usize; self.push(chunk.constants[idx].clone()); ip += 1; }
                2 => { let b = self.pop().as_number(); let a = self.pop().as_number(); self.push(Value::Number(a + b)); }
                3 => { let b = self.pop().as_number(); let a = self.pop().as_number(); self.push(Value::Number(a - b)); }
                6 => { let name = chunk.names[chunk.code[ip] as usize].clone(); let val = self.pop(); self.globals.insert(name, val); ip += 1; }
                7 => { let name = &chunk.names[chunk.code[ip] as usize]; let val = self.globals.get(name).cloned().unwrap_or(Value::Nil); self.push(val); ip += 1; }
                8 => { let val = self.pop(); match val { Value::Number(n) => println!("{}", n), Value::Str(s) => println!("{}", s), _ => {} } }
                10 => { let target = self.read_u16(&chunk.code, &mut ip); let cond = self.pop(); if !cond.is_truthy() { ip = target; } }
                11 => { ip = self.read_u16(&chunk.code, &mut ip); }
                15 => { let r = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() % 5) + 1; self.push(Value::Number(r as f64)); }
                21 => { let ms = self.pop().as_number(); thread::sleep(Duration::from_millis(ms as u64)); }
                22 => { print!("{esc}[2J{esc}[1;1H", esc = 27 as char); io::stdout().flush().unwrap(); } 
                23 => { 
                    io::stdout().flush().unwrap();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).ok();
                    self.push(Value::Str(input.trim().to_string()));
                }
                20 => { // doc_file
                    let p = if let Value::Str(s) = self.pop() { s } else { "save_game.txt".to_string() };
                    let content = fs::read_to_string(p).unwrap_or("0".to_string());
                    self.push(Value::Str(content));
                }
                24 => { // ghi_file: Content pop trước, Path pop sau
                    let content_val = self.pop();
                    let path_val = self.pop();
                    let content = if let Value::Str(s) = content_val { s } else { "0".to_string() };
                    let path = if let Value::Str(s) = path_val { s } else { "save_game.txt".to_string() };
                    fs::write(path, content).ok();
                }
                25 => {
                    let val = self.pop();
                    match val { Value::Number(n) => print!("{}", n), Value::Str(s) => print!("{}", s), _ => {} }
                    io::stdout().flush().unwrap();
                }
                _ => {}
            }
        }
    }
}
