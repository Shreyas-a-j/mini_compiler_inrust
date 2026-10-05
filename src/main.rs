mod lexer;
mod token;
mod ast;
mod parser;
mod parser_test;

use lexer::Lexer;
use token::Token;
use crate::ast::Expression;

fn main() {
    
    // getting command line argument
    let args: Vec<String> = std::env::args().collect();

    // source file handling if incorrect
    let file_path = match args.get(1) {
        Some(path) => path,
        None => {
            eprintln!("Usage: cargo run -- <source-file>");
            return;
        }
    };

    // read the source file
    fn read_source(file_path: &str) -> Result<String, std::io::Error> {
        std::fs::read_to_string(file_path)
    }

    let source = match read_source(file_path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("Could not read file '{}': {}", file_path, error);
            return;
        }
    };

    // source code to lexer
    let mut lexer = Lexer::new(&source);


    // getting the tokens
    loop {
        match lexer.next_token() {
            Ok(Token::EOF) => break,

            Ok(token) => {
                println!("{:?}", token);
            }

            Err(error) => {
                eprintln!("Lexer error: {:?}", error);
                break;
            }
        }
    }
}

