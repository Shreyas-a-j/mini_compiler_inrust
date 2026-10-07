#[derive(Debug, PartialEq)]
pub enum Token {
    Let,
    If,
    Else,
    Return,

    Identifier(String),
    Integer(i64),
    StringLiteral(String),

    Equal,
    EqualEqual,
    NotEqual,

    Plus,
    Minus,
    Star,
    Slash,

    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    LeftBrace,
    RightBrace,

    LeftParen,
    RightParen,

    Semicolon,

    EOF,
}