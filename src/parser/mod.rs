use crate::lexer::{Token, lexer};

#[derive(Debug, PartialEq)]
enum Statement {
    Let{ name: String, value: Expr},
    Return{ value: Expr },
    Expression(Expr),
}

#[derive(Debug, PartialEq)]
enum Expr {
    Const(u32),
    Ident(String),
}

fn parse(mut input: Vec<Token>) -> Vec<Statement> {
    let mut program = vec![];

    loop {
        let token = &input[0];

        match token {
            Token::EOF => break,
            Token::LET => parse_let(&mut input, &mut program),
            Token::RETURN => parse_return(&mut input, &mut program),
            _ => program.push(Statement::Expression(parse_expression(&mut input)))
        }
        assert_eq!(Token::SEMICOLON, input.remove(0));

    }

    program
}

fn parse_let(input: &mut Vec<Token>, program: &mut Vec<Statement>) {
    assert_eq!(Token::LET, input.remove(0));
    let name = match input.remove(0) {
        Token::IDENT(name) => name,
        _ => panic!("parse error at let statement"),
    };
    assert_eq!(Token::ASSIGN, input.remove(0));
    let value = parse_expression(input);
    program.push(Statement::Let {name, value});
}

fn parse_return(input: &mut Vec<Token>, program: &mut Vec<Statement>) {
    assert_eq!(Token::RETURN, input.remove(0));
    let value = match input.remove(0) {
        Token::INT(value) => value,
        _ => panic!("parse error at return statement"),
    };
    program.push(Statement::Return {value: Expr::Const(value)});
}

fn parse_expression(input: &mut Vec<Token>) -> Expr {
    match input.remove(0) {
        Token::INT(value) => Expr::Const(value),
        Token::IDENT(value) => Expr::Ident(value),
        _ => panic!("parse error at let statement"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_let() {
        let input = "let x = 5;";
        let tokens = lexer().parse(input.as_bytes()).unwrap();
        let ast = parse(tokens);

        assert_eq!(
            vec![
                Statement::Let { name: String::from("x"), value: Expr::Const(5) },
            ],
            ast
        );
    }

    #[test]
    fn parse_return() {
        let input = "return 5;";
        let tokens = lexer().parse(input.as_bytes()).unwrap();
        let ast = parse(tokens);

        assert_eq!(
            vec![
                Statement::Return { value: Expr::Const(5) },
            ],
            ast
        );
    }

    #[test]
    fn parse_let_ident() {
        let input = "let myVar = anotherV;";
        let tokens = lexer().parse(input.as_bytes()).unwrap();
        let ast = parse(tokens);

        assert_eq!(
            vec![
                Statement::Let { name: String::from("myVar"), value: Expr::Ident(String::from("anotherV")) },
            ],
            ast
        );
    }

    #[test]
    fn parse_expression_statement() {
        let input = "foo;";
        let tokens = lexer().parse(input.as_bytes()).unwrap();
        let ast = parse(tokens);

        assert_eq!(
            vec![
                Statement::Expression(Expr::Ident(String::from("foo"))),
            ],
            ast
        );
    }
}
