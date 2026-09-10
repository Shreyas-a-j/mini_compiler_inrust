mod lexer;
mod token;
mod ast;
mod parser;

use lexer::Lexer;
use token::Token;

fn main() {
    let mut lexer = Lexer::new(
        r#" let x = 10;

// comment

let y = 20;
        "#
    );

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

    ast::test_let_statement();
}

