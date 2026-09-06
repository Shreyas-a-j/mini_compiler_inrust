#[derive(Debug, PartialEq)]
enum Token {
    Let,
    Identifier(String),
    Integer(i64),

    Equal,
    Plus,
    Semicolon,

    EOF,
}

struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn next_token(&mut self) -> Token {
        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let ch = self.input[self.position];

        self.position += 1;

        match ch {
            '=' => Token::Equal,
            '+' => Token::Plus,
            ';' => Token::Semicolon,
            _ => Token::EOF,
        }
    }
}

fn main() {
    let mut lexer = Lexer::new("+=;");

    println!("{:?}", lexer.next_token());
    println!("{:?}", lexer.next_token());
    println!("{:?}", lexer.next_token());
    println!("{:?}", lexer.next_token());

}

