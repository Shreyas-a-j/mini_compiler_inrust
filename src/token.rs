#[derive(Debug, PartialEq)]
pub enum Token {
    Let,
    If,
    Else,
    Return,

    Identifier(String),
    Integer(i64),

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

    Semicolon,

    EOF,
}