mod lexer;
mod token;
mod error;

mod ast {
    pub mod node;
    pub mod stmt;
    pub mod expr;
}

mod parser {
    pub mod parser;
    pub mod block;
}

use lexer::Lexer;
use parser::parser::Parser;

fn main() {
    let code = r#"
#[curly_brakets]
neu a > 5 {
    in("ok")
}
#[curly_brakets_end]
"#;

    // ===== LEX =====
    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();

    loop {
        let tok = lexer.next_token().unwrap();
        tokens.push(tok.clone());
        if tok == token::Token::EOF {
            break;
        }
    }

    println!("===== TOKENS =====");
    for t in &tokens {
        println!("{:?}", t);
    }

    // ===== PARSE =====
    println!("\n===== PARSER =====");

    let mut parser = Parser::new(tokens);

    match parser.parse() {
        Ok(program) => {
            println!("✅ PARSER OK");
            println!("{:#?}", program);
        }
        Err(e) => {
            println!("❌ PARSER ERROR");
            println!("VN: {}", e.vn);
            println!("EN: {}", e.en);
        }
    }
}
