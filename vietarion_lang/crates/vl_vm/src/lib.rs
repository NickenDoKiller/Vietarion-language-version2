use std::collections::HashMap;
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum Value { Number(f64), Str(String) }

#[derive(Debug, Clone)]
pub struct Chunk { pub code: Vec<u8>, pub constants: Vec<Value>, pub names: Vec<String> }

pub struct VM { stack: Vec<Value>, globals: HashMap<String, Value> }

impl VM {
    pub fn new() -> Self { Self { stack: Vec::new(), globals: HashMap::new() } }
    fn pop(&mut self) -> Value { self.stack.pop().unwrap_or(Value::Number(0.0)) }

    pub fn run(&mut self, chunk: Chunk) {
        let mut ip = 0;
        let code = &chunk.code;
        while ip < code.len() {
            let inst = code[ip]; ip += 1;
            match inst {
                0 => break,
                1 => { let idx = code[ip] as usize; ip += 1; self.stack.push(chunk.constants[idx].clone()); }
                2 => { // Lệnh cộng (+)
                    let b = self.pop(); let a = self.pop();
                    match (a, b) {
                        (Value::Number(n1), Value::Number(n2)) => self.stack.push(Value::Number(n1 + n2)),
                        (Value::Str(s1), Value::Str(s2)) => self.stack.push(Value::Str(format!("{}{}", s1, s2))),
                        (Value::Str(s1), Value::Number(n2)) => self.stack.push(Value::Str(format!("{}{}", s1, n2))),
                        (Value::Number(n1), Value::Str(s2)) => self.stack.push(Value::Str(format!("{}{}", n1, s2))),
                    }
                }
                3 => { let b = self.pop(); let a = self.pop(); if let (Value::Number(n1), Value::Number(n2)) = (a, b) { self.stack.push(Value::Number(n1 - n2)); } }
                6 => { let idx = code[ip] as usize; ip += 1; let val = self.pop(); self.globals.insert(chunk.names[idx].clone(), val); }
                7 => { let idx = code[ip] as usize; ip += 1; let val = self.globals.get(&chunk.names[idx]).cloned().unwrap_or(Value::Number(0.0)); self.stack.push(val); }
                8 => { 
                    let val = self.pop(); 
                    match val { 
                        Value::Number(n) => println!("💬 [Vietarion]: {}", n), 
                        Value::Str(s) => println!("💬 [Vietarion]: {}", s) 
                    }
                }
                10 => { let target = code[ip] as usize; ip += 1; let cond = match self.pop() { Value::Number(n) => n, _ => 0.0 }; if cond == 0.0 { ip = target; } }
                11 => { ip = code[ip] as usize; }
                12 => { let b = self.pop(); let a = self.pop(); if let (Value::Number(n1), Value::Number(n2)) = (a, b) { self.stack.push(Value::Number(if n1 > n2 { 1.0 } else { 0.0 })); } }
                14 => { self.pop(); }
                15 => { 
                    let mut rng = rand::thread_rng();
                    // Sửa lỗi ambiguous: gọi rõ ràng kiểu f64
                    let val: f64 = rng.gen_range(1.0f64..11.0f64).floor();
                    self.stack.push(Value::Number(val));
                }
                _ => {}
            }
        }
    }
}
