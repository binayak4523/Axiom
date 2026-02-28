mod lexer;
mod parser;
mod ast;
mod interpreter;
mod typechecker;
mod types;
mod diagnostic;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use typechecker::TypeChecker;
use std::env;
use std::fs;
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: axiom <file.axi>");
        return;
    }

    let file_path = &args[1];

    if Path::new(file_path)
        .extension()
        .and_then(|s| s.to_str())
        != Some("axi")
    {
        eprintln!("❌ Invalid file type");
        eprintln!("💡 Axiom programs must use the .axi extension");
        return;
    }

    let input = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("❌ Failed to read file");
            eprintln!("→ {}", err);
            return;
        }
    };

    run_program(&input);
}

fn run_program(input: &str) {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse();

    let mut typechecker = TypeChecker::new();
    if let Err(diag) = typechecker.check(&program) {
        print_diagnostic(diag);
        return;
    }

    let mut interpreter = Interpreter::new();
    let result = interpreter.execute(&program);

    println!("Result: {:?}", result);
}

fn print_diagnostic(d: crate::diagnostic::Diagnostic) {
    println!("\n❌ {}", d.title);
    println!("→ {}", d.message);
    if let Some(help) = d.help {
        println!("💡 {}", help);
    }
}

