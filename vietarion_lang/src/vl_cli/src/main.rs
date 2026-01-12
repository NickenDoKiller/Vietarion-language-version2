use clap::{Parser, Subcommand};
use colored::*;
use std::fs;
use std::io::Write;
use vl_core::lexer::Lexer;
use vl_core::parser::Parser as VlParser;
use vl_core::compiler::Compiler;
use vl_vm::VM;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scan { file: String },
    Run { file: String },
    Build { file: String }, 
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Build { file } => {
            println!("{} {}", "📦 ĐANG BUILD FILE .vlbc:".blue().bold(), file);
            let content = fs::read_to_string(file).expect("Không đọc được file .vl");
            
            let lexer = Lexer::new(&content);
            let mut parser = VlParser::new(lexer).unwrap();
            let ast = parser.parse_program().unwrap();
            let mut compiler = Compiler::new();
            let chunk = compiler.compile(ast);

            // Ghi file bytecode (.vlbc)
            let out_name = file.replace(".vl", ".vlbc");
            let mut f = fs::File::create(&out_name).expect("Không tạo được file");
            
            // Format: [Số lượng hằng số] [Dữ liệu hằng số] [Mã lệnh]
            f.write_all(&(chunk.constants.len() as u32).to_le_bytes()).unwrap();
            for c in chunk.constants {
                f.write_all(&c.to_le_bytes()).unwrap();
            }
            f.write_all(&chunk.code).unwrap();

            println!("{} {}","✅ Đã xuất file:".green().bold(), out_name.cyan());
            println!("👉 Thử gõ 'hexdump -C {}' để xem nội dung nhị phân!", out_name);
        },
        Commands::Run { file } => {
            println!("{} {}", "🔨 RUN:".yellow().bold(), file);
            let content = fs::read_to_string(file).unwrap();
            let lexer = Lexer::new(&content);
            let mut parser = VlParser::new(lexer).unwrap();
            let ast = parser.parse_program().unwrap();
            let mut compiler = Compiler::new();
            let chunk = compiler.compile(ast);
            let mut vm = VM::new();
            vm.run(chunk);
        }
        _ => {}
    }
}
