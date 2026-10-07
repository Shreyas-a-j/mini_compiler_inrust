use crate::token::Token;
use crate::ast::{BinaryExpression, BinaryOperator, Expression};

#[derive(Debug, PartialEq)]
pub enum ParserError {
    Expected(String),
    Found(String),
}

#[derive(Debug, PartialEq)]
pub struct Parser {
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

    pub fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        let left = self.parse_primary()?;

        if let Some(operator) = self.parse_binary_operator() {
            self.advance();

            let right = self.parse_primary()?;

            return Ok(Expression::Binary(
                BinaryExpression {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                }
            ));
        }

        Ok(left)
    }

    pub fn parse_primary(&mut self) -> Result<Expression, ParserError> {
        match self.current() {
            Some(Token::Integer(value)) => {
                let expression = Expression::Integer(*value);
                self.advance();
                Ok(expression)
            }

            Some(Token::Identifier(name)) => {
                let expression = Expression::Identifier(name.clone());
                self.advance();
                Ok(expression)
            }

            _=> {
                Err(ParserError::Expected("expression".to_string()))
            }
        }
    }

    pub fn parse_binary_operator(&self) -> Option<BinaryOperator> {
        match self.current() {
            Some(Token::Plus) => Some(BinaryOperator::Plus),
            Some(Token::Minus) => Some(BinaryOperator::Minus),
            Some(Token::Star) => Some(BinaryOperator::Star),
            Some(Token::Slash) => Some(BinaryOperator::Slash),

            Some(Token::EqualEqual) => Some(BinaryOperator::EqualEqual),
            Some(Token::NotEqual) => Some(BinaryOperator::NotEqual),
            Some(Token::Less) => Some(BinaryOperator::Less),
            Some(Token::LessEqual) => Some(BinaryOperator::LessEqual),
            Some(Token::Greater) => Some(BinaryOperator::Greater),
            Some(Token::GreaterEqual) => Some(BinaryOperator::GreaterEqual),

            _ => None,
        }
    }

}