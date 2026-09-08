mod lexer;
mod token;

use lexer::Lexer;
use token::Token;

fn main() {
    let mut lexer = Lexer::new(
        "Let x = \"Hello World\";"
    );

    loop {
        match lexer.next_token() {
            Ok(Token::EOF) => break,

            Ok(token) => {
                println!("{:?}", token);
            }

            Err(error) => {
                println!("Lexer error: {:?}", error);
                break;
            }
        }
    }
}

