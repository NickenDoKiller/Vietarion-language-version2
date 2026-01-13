use std::fs;
use std::env;
use vl_core::lexer::Lexer;
use vl_core::parser::Parser;
use vl_core::compiler::Compiler;
use vl_vm::VM;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Cú pháp: cargo run -p vl_cli -- run <file_name>");
        return;
    }

    let file_path = &args[2];
    let source = fs::read_to_string(file_path).expect("Không đọc được file");

    // 1. Lexing
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.scan_tokens(); 

    // 2. Parsing
    let mut parser = Parser::new(tokens);
    let stmts = match parser.parse() {
        Ok(s) => s,
        Err(e) => {
            println!("Lỗi Parser: {:?}", e);
            return;
        }
    };

    // 3. Compiling
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(stmts);

    // 4. Running
    let mut vm = VM::new();
    vm.run(chunk);
}
