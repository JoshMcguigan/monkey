use crate::parser::Statement;
use crate::parser::Expr;
use crate::parser::Prefix;
use crate::parser::Operator;

#[derive(Debug, PartialEq)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    Null,
}

fn eval_expr(expression: Expr) -> Object {
    match expression {
        Expr::Const(num) => Object::Integer(num),
        Expr::Boolean(val) => Object::Boolean(val),
        Expr::Prefix { prefix: Prefix::Bang, value: expr } => {
            match *expr {
                Expr::Boolean(val) => Object::Boolean(!val),
                _ => panic!("! operator only valid for boolean type"),
            }
        },
        Expr::Prefix { prefix: Prefix::Minus, value: expr } => {
            match *expr {
                Expr::Const(val) => Object::Integer(-val),
                _ => panic!("minus operator only valid for integer type"),
            }
        },
        Expr::Infix { left, operator: Operator::Plus, right } => {
            match (*left, *right) {
                (Expr::Const(left), Expr::Const(right)) => Object::Integer(left + right),
                _ => panic!("plus operator only valid on integer types")
            }
        },
        Expr::Infix { left, operator: Operator::Minus, right } => {
            match (*left, *right) {
                (Expr::Const(left), Expr::Const(right)) => Object::Integer(left - right),
                _ => panic!("minus operator only valid on integer types")
            }
        },
        Expr::Infix { left, operator: Operator::Multiply, right } => {
            match (*left, *right) {
                (Expr::Const(left), Expr::Const(right)) => Object::Integer(left * right),
                _ => panic!("multiply operator only valid on integer types")
            }
        },
        Expr::Infix { left, operator: Operator::Divide, right } => {
            match (*left, *right) {
                (Expr::Const(left), Expr::Const(right)) => Object::Integer(left / right),
                _ => panic!("divide operator only valid on integer types")
            }
        },
        Expr::Infix { left, operator: Operator::LessThan, right } => {
            match (*left, *right) {
                (Expr::Const(left), Expr::Const(right)) => Object::Boolean(left < right),
                _ => panic!("less than operator only valid on integer types")
            }
        },
        Expr::Infix { left, operator: Operator::GreaterThan, right } => {
            match (*left, *right) {
                (Expr::Const(left), Expr::Const(right)) => Object::Boolean(left > right),
                _ => panic!("greater than operator only valid on integer types")
            }
        },
        Expr::Infix { left, operator: Operator::Equals, right } => {
            match (*left, *right) {
                (Expr::Const(left), Expr::Const(right)) => Object::Boolean(left == right),
                (Expr::Boolean(left), Expr::Boolean(right)) => Object::Boolean(left == right),
                _ => panic!("equals operator used on invalid types")
            }
        },
        Expr::Infix { left, operator: Operator::NotEquals, right } => {
            match (*left, *right) {
                (Expr::Const(left), Expr::Const(right)) => Object::Boolean(left != right),
                (Expr::Boolean(left), Expr::Boolean(right)) => Object::Boolean(left != right),
                _ => panic!("not equals operator used on invalid types")
            }
        },
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
        test_eval("5;", Object::Integer(5));
    }

    #[test]
    fn eval_bool() {
        test_eval("true;", Object::Boolean(true));
        test_eval("false;", Object::Boolean(false));
    }

    #[test]
    fn eval_bang() {
        test_eval("!true;", Object::Boolean(false));
        test_eval("!false;", Object::Boolean(true));
    }

    #[test]
    fn eval_negative() {
        test_eval("-5;", Object::Integer(-5));
    }

    #[test]
    fn eval_infix() {
        test_eval("5 + 5;", Object::Integer(10));
        test_eval("5 - 5;", Object::Integer(0));
        test_eval("5 * 5;", Object::Integer(25));
        test_eval("5 / 5;", Object::Integer(1));
        test_eval("5 > 1;", Object::Boolean(true));
        test_eval("5 < 1;", Object::Boolean(false));
        test_eval("5 == 1;", Object::Boolean(false));
        test_eval("5 != 1;", Object::Boolean(true));
        test_eval("true == true;", Object::Boolean(true));
        test_eval("true != true;", Object::Boolean(false));
//        test_eval("(1 > 2) == false;", Object::Boolean(true));
    }

    fn test_eval(input: &str, expected: Object) {
        let mut tokens = lexer().parse(input.as_bytes()).unwrap();
        let ast = parse(&mut tokens);
        let obj = eval_statements(ast);

        assert_eq!(
            expected,
            obj
        );
    }
}
