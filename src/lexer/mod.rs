use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
#[logos(trivia = r"\p{Whitespace}")]
pub enum Token {
    #[error]
    ERROR,
    EOF,
    #[regex("[a-zA-Z]+", |lexer| lexer.slice().to_owned())]
    IDENT(String),
    #[regex("[0-9]+", |lexer| lexer.slice().parse())]
    INT(i32),
    #[regex(r#""[^"]*""#, |lexer| lexer.slice()[1..(lexer.slice().len()-1)].to_owned())]
    STRING(String), // string literal, let x = "my string";
    #[token = "="]
    ASSIGN,
    #[token = "+"]
    PLUS,
    #[token = "-"]
    MINUS,
    #[token = "/"]
    SLASH,
    #[token = "*"]
    ASTERISK,
    #[token = "<"]
    LT,
    #[token = ">"]
    GT,
    #[token = "!"]
    BANG,
    #[token = ","]
    COMMA,
    #[token = ";"]
    SEMICOLON,
    #[token = "("]
    LPAREN,
    #[token = ")"]
    RPAREN,
    #[token = "{"]
    LBRACE,
    #[token = "}"]
    RBRACE,
    #[token = "fn"]
    FUNCTION,
    #[token = "let"]
    LET,
    #[token = "if"]
    IF,
    #[token = "else"]
    ELSE,
    #[token = "return"]
    RETURN,
    #[token = "true"]
    TRUE,
    #[token = "false"]
    FALSE,
    #[token = "=="]
    EQ,
    #[token = "!="]
    NOT_EQ,
}

// TODO this shouldn't be a result type
pub fn lex(input: &str) -> Result<Vec<Token>, ()> {
    let mut tokens = Token::lexer(input)
        .collect::<Vec<Token>>();
    tokens.push(Token::EOF);

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_tokens() {
        let input = "=+(){},;";
        let tokens = lex(input);

        assert_eq!(
            vec![
                Token::ASSIGN,
                Token::PLUS,
                Token::LPAREN,
                Token::RPAREN,
                Token::LBRACE,
                Token::RBRACE,
                Token::COMMA,
                Token::SEMICOLON,
                Token::EOF,
            ],
            tokens.unwrap()
        );
    }

    #[test]
    fn lex_let() {
        let input = "let five = 5;";
        let tokens = lex(input);

        assert_eq!(
            vec![
                Token::LET,
                Token::IDENT(String::from("five")),
                Token::ASSIGN,
                Token::INT(5),
                Token::SEMICOLON,
                Token::EOF,
            ],
            tokens.unwrap()
        );
    }

    #[test]
    fn lex_let_ident_contains_keyword() {
        let input = "let letter = 5;";
        let tokens = lex(input);

        assert_eq!(
            vec![
                Token::LET,
                Token::IDENT(String::from("letter")),
                Token::ASSIGN,
                Token::INT(5),
                Token::SEMICOLON,
                Token::EOF,
            ],
            tokens.unwrap()
        );
    }

    #[test]
    fn lex_ident_ending_with_semicolon() {
        let input = "let ten = 5 + five;";
        let tokens = lex(input);

        assert_eq!(
            vec![
                Token::LET,
                Token::IDENT(String::from("ten")),
                Token::ASSIGN,
                Token::INT(5),
                Token::PLUS,
                Token::IDENT(String::from("five")),
                Token::SEMICOLON,
                Token::EOF,
            ],
            tokens.unwrap()
        );
    }

    #[test]
    fn lex_function() {
        let input = r#"
            let add = fn(x, y) {
              x + y;
            };
        "#;
        let tokens = lex(input);

        assert_eq!(
            vec![
                Token::LET,
                Token::IDENT(String::from("add")),
                Token::ASSIGN,
                Token::FUNCTION,
                Token::LPAREN,
                Token::IDENT(String::from("x")),
                Token::COMMA,
                Token::IDENT(String::from("y")),
                Token::RPAREN,
                Token::LBRACE,
                Token::IDENT(String::from("x")),
                Token::PLUS,
                Token::IDENT(String::from("y")),
                Token::SEMICOLON,
                Token::RBRACE,
                Token::SEMICOLON,
                Token::EOF,
            ],
            tokens.unwrap()
        );
    }

    #[test]
    fn lex_function_call() {
        let input = "let result = add(five, ten);";
        let tokens = lex(input);

        assert_eq!(
            vec![
                Token::LET,
                Token::IDENT(String::from("result")),
                Token::ASSIGN,
                Token::IDENT(String::from("add")),
                Token::LPAREN,
                Token::IDENT(String::from("five")),
                Token::COMMA,
                Token::IDENT(String::from("ten")),
                Token::RPAREN,
                Token::SEMICOLON,
                Token::EOF,
            ],
            tokens.unwrap()
        );
    }

    #[test]
    fn lex_additional_opeations() {
        let input = "- / * < > !";
        let tokens = lex(input);

        assert_eq!(
            vec![
                Token::MINUS,
                Token::SLASH,
                Token::ASTERISK,
                Token::LT,
                Token::GT,
                Token::BANG,
                Token::EOF,
            ],
            tokens.unwrap()
        );
    }

    #[test]
    fn lex_additional_keywords() {
        let input = r#"
            if (x) {
                return true;
            } else {
                return false;
            }
        "#;
        let tokens = lex(input);

        assert_eq!(
            vec![
                Token::IF,
                Token::LPAREN,
                Token::IDENT(String::from("x")),
                Token::RPAREN,
                Token::LBRACE,
                Token::RETURN,
                Token::TRUE,
                Token::SEMICOLON,
                Token::RBRACE,
                Token::ELSE,
                Token::LBRACE,
                Token::RETURN,
                Token::FALSE,
                Token::SEMICOLON,
                Token::RBRACE,
                Token::EOF,
            ],
            tokens.unwrap()
        );
    }

    #[test]
    fn lex_equal_not_equal() {
        let input = r#"
            10 == 10;
            10 != 9;
        "#;
        let tokens = lex(input);

        assert_eq!(
            vec![
                Token::INT(10),
                Token::EQ,
                Token::INT(10),
                Token::SEMICOLON,
                Token::INT(10),
                Token::NOT_EQ,
                Token::INT(9),
                Token::SEMICOLON,
                Token::EOF,
            ],
            tokens.unwrap()
        );
    }

    #[test]
    fn lex_string() {
        let input = r#"let words = "foo bar";"#;
        let tokens = lex(input);

        assert_eq!(
            vec![
                Token::LET,
                Token::IDENT(String::from("words")),
                Token::ASSIGN,
                Token::STRING(String::from("foo bar")),
                Token::SEMICOLON,
                Token::EOF,
            ],
            tokens.unwrap()
        );
    }

}
