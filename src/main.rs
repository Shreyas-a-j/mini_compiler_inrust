mod lexer;
mod token;

use lexer::Lexer;
use token::Token;

fn main() {
    let mut lexer = Lexer::new(
        "let x = 10 + 20 * 3;

        if x >= 50 {
            return x;
        }"
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

