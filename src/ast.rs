#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Star,
    Slash,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Integer(i64),
    Identifier(String),
    Binary(BinaryExpression),
}

#[derive(Debug, PartialEq)]
struct BinaryExpression {
    left: Box<Expression>,
    operator: BinaryOperator,
    right: Box<Expression>,
}

#[derive(Debug, PartialEq)]
struct LetStatement {
    name: String,
    value: Expression,
}

pub fn test_let_statement() {
    let statement = LetStatement {
        name: "y".to_string(),

        value: Expression::Binary(
            BinaryExpression {
                left: Box::new(Expression::Integer(3)),
                operator: BinaryOperator::Plus,
                right: Box::new(Expression::Integer(4)),
            }
        ),
    };

    println!("{:?}",statement.value);
}

