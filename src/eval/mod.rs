use crate::parser::Statement;
use crate::parser::Expr;
use crate::parser::Prefix;
use crate::parser::Operator;

mod env;
pub use self::env::Env;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Integer(i32),
    String(String),
    Boolean(bool),
    Null,
    Return(Box<Object>),
    Function{parameters: Vec<String>, body: Vec<Statement>}
}

fn eval_expr(expression: Expr, env: &mut Env) -> Object {
    match expression {
        Expr::String(string) => Object::String(string),
        Expr::Const(num) => Object::Integer(num),
        Expr::Boolean(val) => Object::Boolean(val),
        Expr::Prefix { prefix: Prefix::Bang, value: expr } => {
            match eval_expr(*expr, env) {
                Object::Boolean(val) => Object::Boolean(!val),
                _ => panic!("! operator only valid for boolean type"),
            }
        },
        Expr::Prefix { prefix: Prefix::Minus, value: expr } => {
            match eval_expr(*expr, env) {
                Object::Integer(val) => Object::Integer(-val),
                _ => panic!("minus operator only valid for integer type"),
            }
        },
        Expr::Infix { left, operator: Operator::Plus, right } => {
            match (eval_expr(*left, env), eval_expr(*right, env)) {
                (Object::Integer(left), Object::Integer(right)) => Object::Integer(left + right),
                (Object::String(left), Object::String(right)) => Object::String(left + &right),
                _ => panic!("plus operator used on invalid types")
            }
        },
        Expr::Infix { left, operator: Operator::Minus, right } => {
            match (eval_expr(*left, env), eval_expr(*right, env)) {
                (Object::Integer(left), Object::Integer(right)) => Object::Integer(left - right),
                _ => panic!("minus operator only valid on integer types")
            }
        },
        Expr::Infix { left, operator: Operator::Multiply, right } => {
            match (eval_expr(*left, env), eval_expr(*right, env)) {
                (Object::Integer(left), Object::Integer(right)) => Object::Integer(left * right),
                _ => panic!("multiply operator only valid on integer types")
            }
        },
        Expr::Infix { left, operator: Operator::Divide, right } => {
            match (eval_expr(*left, env), eval_expr(*right, env)) {
                (Object::Integer(left), Object::Integer(right)) => Object::Integer(left / right),
                _ => panic!("divide operator only valid on integer types")
            }
        },
        Expr::Infix { left, operator: Operator::LessThan, right } => {
            match (eval_expr(*left, env), eval_expr(*right, env)) {
                (Object::Integer(left), Object::Integer(right)) => Object::Boolean(left < right),
                _ => panic!("less than operator only valid on integer types")
            }
        },
        Expr::Infix { left, operator: Operator::GreaterThan, right } => {
            match (eval_expr(*left, env), eval_expr(*right, env)) {
                (Object::Integer(left), Object::Integer(right)) => Object::Boolean(left > right),
                _ => panic!("greater than operator only valid on integer types")
            }
        },
        Expr::Infix { left, operator: Operator::Equals, right } => {
            match (eval_expr(*left, env), eval_expr(*right, env)) {
                (Object::Integer(left), Object::Integer(right)) => Object::Boolean(left == right),
                (Object::Boolean(left), Object::Boolean(right)) => Object::Boolean(left == right),
                _ => panic!("equals operator used on invalid types")
            }
        },
        Expr::Infix { left, operator: Operator::NotEquals, right } => {
            match (eval_expr(*left, env), eval_expr(*right, env)) {
                (Object::Integer(left), Object::Integer(right)) => Object::Boolean(left != right),
                (Object::Boolean(left), Object::Boolean(right)) => Object::Boolean(left != right),
                _ => panic!("not equals operator used on invalid types")
            }
        },
        Expr::If { condition, consequence, alternative } => {
            if eval_expr(*condition, env) == Object::Boolean(true) {
                eval_statements(consequence, env)
            } else {
                eval_statements(alternative, env)
            }
        },
        Expr::Ident(name) => env.get(&name).expect("attempted access to invalid binding"),
        Expr::Function{parameters, body} => Object::Function {parameters, body},
        Expr::Call{function, arguments} => {
            let (parameters, body) = match *function {
                Expr::Ident(func_name) => {
                    match env.get(&func_name).expect("tried to call function which was not defined") {
                        Object::Function {parameters, body} => (parameters, body),
                        _ => panic!("attempted to call non-function"),
                    }
                }
                Expr::Function {parameters, body} => (parameters, body),
                _ => panic!("attempted to call non-function"),
            };

            assert_eq!(parameters.len(), arguments.len(), "called function with wrong number of parameters");

            let mut env_func = Env::new();
            for (parameter, arg_value) in parameters.into_iter().zip(arguments.into_iter()) {
                env_func.set(parameter, eval_expr(arg_value, env));
            }

            eval_return_scope(body, &mut env_func)
        },
    }
}

fn eval_statement(statement: Statement, env: &mut Env) -> Object {
    match statement {
        Statement::Expression(expr) => eval_expr(expr, env),
        Statement::Let{name, value} => {
            let value = eval_expr(value, env);
            env.set(name, value.clone());
            value
        },
        Statement::Return{value: expr} => Object::Return(Box::new(eval_expr(expr, env))),
    }
}

/// similar to eval_return_scope but doesn't unwrap Return types
/// useful for if-else blocks where the return should return from the parent scope as well
fn eval_statements(statements: Vec<Statement>, env: &mut Env) -> Object {
    let mut result = Object::Null;

    for statement in statements {
        result = eval_statement(statement, env);

        if let &Object::Return(_) = &result {
            return result;
        }
    }

    result
}

pub fn eval_return_scope(statements: Vec<Statement>, env: &mut Env) -> Object {
    let result = eval_statements(statements, env);

    match result {
        // unwrap Return type
        Object::Return(res) => *res,
        _ => result,
    }
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
    fn eval_string_literal() {
        test_eval(r#""foo bar";"#, Object::String(String::from("foo bar")));
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
        test_eval("!(1 > 2);", Object::Boolean(true));
    }

    #[test]
    fn eval_negative() {
        test_eval("-5;", Object::Integer(-5));
        test_eval("-(1 - 2);", Object::Integer(1));
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
        test_eval("(1 > 2) == false;", Object::Boolean(true));
    }

    #[test]
    fn eval_infix_string() {
        test_eval(r#""hello " + "world";"#, Object::String(String::from("hello world")));
    }

    #[test]
    fn eval_infix_nested_types() {
        test_eval("(1 + 2) + 3;", Object::Integer(6));
        test_eval("(1 + 2) - 3;", Object::Integer(0));
        test_eval("(1 + 2) * 3;", Object::Integer(9));
        test_eval("(1 + 2) / 3;", Object::Integer(1));
        test_eval("(1 + 2) < 3;", Object::Boolean(false));
        test_eval("(1 + 2) > 3;", Object::Boolean(false));
        test_eval("(1 > 2) == false;", Object::Boolean(true));
        test_eval("(1 > 2) != false;", Object::Boolean(false));
    }

    #[test]
    fn eval_if() {
        test_eval("if (true) { 10; };", Object::Integer(10));
        test_eval("if (false) { 10; };", Object::Null);
        test_eval("if (false) { 10; } else { 11; };", Object::Integer(11));
        test_eval("if (1 > 2) { 10; } else { 11; };", Object::Integer(11));
        test_eval("if (1 < 2) { 10; } else { 11; };", Object::Integer(10));
    }

    #[test]
    fn eval_return() {
        test_eval("return 10;", Object::Integer(10));
        test_eval("return 10; 11;", Object::Integer(10));
        test_eval("9; return 2 * 5; 9;", Object::Integer(10));
        test_eval(r#"
            if (10 > 1) {
              if (10 > 1) {
                return 10;
              };

              return 1;
            };
        "#, Object::Integer(10));
    }

    #[test]
    fn eval_binding() {
        test_eval("let a = 10; a;", Object::Integer(10));
        test_eval("let a = 5 * 5; a;", Object::Integer(25));
        test_eval("let a = 5 * 5; let b = a; b;", Object::Integer(25));
        test_eval("let a = 5; let b = a; let c = a + b + 5; c;", Object::Integer(15));
        test_eval("let a = 10;", Object::Integer(10)); // useful for repl
    }

    #[test]
    fn eval_function() {
        test_eval("fn(x) { x; };", Object::Function {
            parameters: vec![String::from("x")],
            body: vec![Statement::Expression(Expr::Ident(String::from("x")))]
        });
        test_eval("let identity = fn(x) { x; }; identity(5);", Object::Integer(5));
        test_eval("let identity = fn(x) { return x; }; identity(5);", Object::Integer(5));
        test_eval("let double = fn(x) { x * 2; }; double(5);", Object::Integer(10));
        test_eval("let add = fn(x, y) { x + y; }; add(5, 5);", Object::Integer(10));
        test_eval("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", Object::Integer(20));
        test_eval("let add = fn(x, y) { return x + y; }; let three = add(1, 2); 5;", Object::Integer(5)); // return value inside the function should not cause the entire program to return
    }

    fn test_eval(input: &str, expected: Object) {
        let mut tokens = lexer().parse(input.as_bytes()).unwrap();
        let ast = parse(&mut tokens);
        let mut env = Env::new();
        let obj = eval_return_scope(ast, &mut env);

        assert_eq!(
            expected,
            obj
        );
    }
}
