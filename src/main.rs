use VYL::lexer::Lexer;
use VYL::parser::Parser;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        std::process::exit(1);
    }

    let source_path = &args[1]; 
    let source_code = match fs::read_to_string(source_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", source_path, e);
            std::process::exit(1);
        }
    };
 
    let mut lexer = Lexer::new(&source_code); 
    let tokens = lexer.tokenize();
    
    println!("Tokens:");
    for token in &tokens {
        println!("{:?}", token);
    }
    
    let mut parser = Parser::new(tokens);    
    match parser.parse() {
        Ok(ast) => {
            println!("\nAST:");
            println!("{:#?}", ast);
            
            // TODO: Semantic analysis
            // TODO: Code generation
        },
        Err(e) => {
            eprintln!("Error parsing: {}", e);
            std::process::exit(1);
        }
    }
}
