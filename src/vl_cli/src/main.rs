use std::env;
use std::fs;
use vl_core::lexer::Lexer;
use vl_core::parser::Parser;
use vl_core::compiler::Compiler;
use vl_vm::VM;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Cách dùng: cargo run -p vl_cli -- run <file>");
        return;
    }

    let filename = &args[2];
    let source = fs::read_to_string(filename).expect("Không đọc được file");

    // BƯỚC 1: LEXER biến chuỗi thành danh sách Tokens
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    // BƯỚC 2: PARSER biến Tokens thành AST (Cây cú pháp)
    let mut parser = Parser::new(tokens);
    let stmts = match parser.parse() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Lỗi cú pháp: {:?} tại dòng {}", e.msg_vi, e.line);
            return;
        }
    };

    // BƯỚC 3: COMPILER biến AST thành Bytecode (Chunk)
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(stmts);

    // BƯỚC 4: VM thực thi Bytecode
    let mut vm = VM::new();
    vm.run(chunk);
}
