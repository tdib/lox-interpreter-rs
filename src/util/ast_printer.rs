use crate::expression::Expression;
use crate::token::Literal;

pub trait AstPrinter {
    fn format_ast(expression: &Expression) -> String;
}

impl AstPrinter for Expression {
    fn format_ast(expression: &Expression) -> String {
        match expression {
            Expression::Binary {
                left,
                operator,
                right,
            } => parenthesise(operator.lexeme.clone(), &[*left.clone(), *right.clone()]),
            Expression::Grouping { expression } => {
                parenthesise("group".to_string(), &[*expression.clone()])
            }
            Expression::Literal { value } => match value {
                Literal::String(str) => str.to_string(),
                Literal::Number(num) => num.to_string(),
                Literal::Boolean(bool) => bool.to_string(),
                Literal::None => "nil".to_string(),
            },
            Expression::Unary { operator, right } => {
                parenthesise(operator.lexeme.clone(), &[*right.clone()])
            }
        }
    }
}

fn parenthesise(name: String, expressions: &[Expression]) -> String {
    let mut builder = String::new();
    builder.push('(');
    builder.push_str(&name);

    for expression in expressions {
        builder.push(' ');
        builder.push_str(&expression.to_string())
    }

    builder.push(')');
    builder
}
