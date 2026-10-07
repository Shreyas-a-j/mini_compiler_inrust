use crate::token::Token;
use crate::parser::Parser;
use crate::ast::Expression;

#[test]
pub fn parser_integer_expression() {
    let tokens = vec![
        Token::Integer(42),
        Token::EOF,
    ];

    let mut parser = Parser::new(tokens);

    let expression = parser.parse_expression();

    assert_eq!(
        expression,
        Ok(Expression::Integer(42))
    );
}

pub fn parser_binary_expression() {
    let tokens = vec![
        Token::Integer(10),
        Token::Plus,
        Token::Integer(20),
        Token::EOF,
    ];

    let mut parser = Parser::new(tokens);

    let expression = parser.parse_expression();

    println!("{:#?}",expression);
}