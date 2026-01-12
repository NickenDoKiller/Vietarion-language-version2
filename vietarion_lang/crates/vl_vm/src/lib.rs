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
