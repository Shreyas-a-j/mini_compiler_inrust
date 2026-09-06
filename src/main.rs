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

fn main() {
    let token = Token::Let;

    println!("{:?}", token);
}