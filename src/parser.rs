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
        self.parse_binary_expression(0)
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

            Some(Token::LeftParen) => {
                self.advance();

                let expression = self.parse_expression()?;

                match self.current() {
                    Some(Token::RightParen) => {
                        self.advance();
                        Ok(expression)
                    }

                    _ => Err(ParserError::Expected(
                    "closing ')'".to_string()
                    )),
                }
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

    pub fn parse_binary_expression( &mut self,
        min_precedence: u8,
        )-> Result<Expression, ParserError> {
            let mut left = self.parse_primary()?;

            loop {
                let operator = match self.parse_binary_operator() {
                Some(operator) => operator,
                None => break,
            };

            let operator_precedence = Self::precedence(&operator);

            if operator_precedence < min_precedence {
                break;
            }

            self.advance();

            let right = self.parse_binary_expression(operator_precedence + 1)?;

            left = Expression::Binary(
                BinaryExpression {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                }
            );
        }

        Ok(left)
    }

    pub fn precedence(operator: &BinaryOperator) -> u8 {
        match operator {
            BinaryOperator::Star | BinaryOperator::Slash => 3,

            BinaryOperator::Plus | BinaryOperator::Minus => 2,

            BinaryOperator::EqualEqual
            | BinaryOperator::NotEqual
            | BinaryOperator::Less
            | BinaryOperator::LessEqual
            | BinaryOperator::Greater
            | BinaryOperator::GreaterEqual => 1,
        }
    }

}