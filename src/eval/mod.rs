use crate::parser::Statement;
use crate::parser::Expr;

#[derive(Debug, PartialEq)]
pub enum Object {
    Integer(u32),
    Null
}

fn eval_expr(expression: Expr) -> Object {
    match expression {
        Expr::Const(num) => Object::Integer(num),
        _ => panic!("eval expr not implemented for this type")
    }
}

pub fn eval_statements(statements: Vec<Statement>) -> Object {
    let mut result = Object::Null;

    for statement in statements {
        result = match statement {
            Statement::Expression(expr) => eval_expr(expr),
            _ => Object::Null,
        };
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lexer;
    use crate::parser::parse;

    #[test]
    fn eval_int_literal() {
        let input = "5;";
        let mut tokens = lexer().parse(input.as_bytes()).unwrap();
        let ast = parse(&mut tokens);
        let obj = eval_statements(ast);

        assert_eq!(
            Object::Integer(5),
            obj
        );
    }
}
