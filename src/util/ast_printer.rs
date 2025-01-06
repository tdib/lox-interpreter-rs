use crate::expression::Expression;
use crate::token::Literal;

pub trait Printer {
    fn to_string(expression: Expression) -> String;
}

impl Printer for Expression {
    fn to_string(expression: Expression) -> String {
        match expression {
            Expression::Binary {
                left,
                operator,
                right,
            } => parenthesise(operator.lexeme, &[*left, *right]),
            Expression::Grouping { expression } => {
                parenthesise("group".to_string(), &[*expression])
            }
            Expression::Literal { value } => match value {
                Literal::String(str) => str.to_string(),
                Literal::Number(num) => num.to_string(),
                Literal::Boolean(bool) => bool.to_string(),
                Literal::None => "nil".to_string(),
            },
            Expression::Unary { operator, right } => parenthesise(operator.lexeme, &[*right]),
        }
    }
}

fn parenthesise(name: String, expressions: &[Expression]) -> String {
    let mut builder = String::new();
    builder.push('(');
    builder.push_str(&name);

    for expression in expressions {
        builder.push(' ');
        builder.push_str(&Expression::to_string(expression.clone()))
    }

    builder.push(')');
    builder
}
