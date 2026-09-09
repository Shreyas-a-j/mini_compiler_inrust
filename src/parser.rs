use crate::token::Token;

#[derive(Debug, PartialEq)]
struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            position: 0,
        }
    }

    pub fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    pub fn peek(&self) ->  Option<&Token> {
        self.tokens.get(self.position + 1)
    }

    pub fn advance(&mut self){
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }
}