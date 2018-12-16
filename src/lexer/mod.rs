use pom::parser::{Parser, seq, one_of, is_a, not_a, end};

use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum Token {
    EOF,
    IDENT(String),
    INT(u32),
    ASSIGN,
    PLUS,
    MINUS,
    SLASH,
    ASTERISK,
    LT,
    GT,
    BANG,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    IF,
    ELSE,
    RETURN,
    TRUE,
    FALSE,
    EQ,
    NOT_EQ,
}

fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(1..).discard()
}

fn lex_keyword<'a>(match_str: &'static str, token: Token) -> Parser<'a, u8, Token> {
    seq(match_str.as_bytes()).map(move |_| token.clone())
        - -not_a(|byte| char::from(byte).is_alphabetic())
}

fn lex_ident<'a>() -> Parser<'a, u8, Token> {
    is_a(|byte| char::from(byte).is_alphabetic())
        .repeat(1..).convert(String::from_utf8)
        .map(|ident| Token::IDENT(ident) )
}

fn lex_int<'a>() -> Parser<'a, u8, Token> {
    is_a(|byte| char::from(byte).is_numeric())
        .repeat(1..)
        .convert(String::from_utf8)
        .convert(|num|u32::from_str(&num))
        .map(|num| Token::INT(num) )
}

fn lex_token<'a>() -> Parser<'a, u8, Token> {
    space().opt() * (
          lex_keyword("let", Token::LET)
        | lex_keyword("fn", Token::FUNCTION)
        | lex_keyword("if", Token::IF)
        | lex_keyword("else", Token::ELSE)
        | lex_keyword("return", Token::RETURN)
        | lex_keyword("true", Token::TRUE)
        | lex_keyword("false", Token::FALSE)
        | lex_ident()
        | lex_int()
        | seq(b"==").map(|_| Token::EQ)
        | seq(b"!=").map(|_| Token::NOT_EQ)
        | seq(b"=").map(|_| Token::ASSIGN)
        | seq(b"+").map(|_| Token::PLUS)
        | seq(b"-").map(|_| Token::MINUS)
        | seq(b"/").map(|_| Token::SLASH)
        | seq(b"*").map(|_| Token::ASTERISK)
        | seq(b"<").map(|_| Token::LT)
        | seq(b">").map(|_| Token::GT)
        | seq(b"!").map(|_| Token::BANG)
        | seq(b"(").map(|_| Token::LPAREN)
        | seq(b")").map(|_| Token::RPAREN)
        | seq(b"{").map(|_| Token::LBRACE)
        | seq(b"}").map(|_| Token::RBRACE)
        | seq(b",").map(|_| Token::COMMA)
        | seq(b";").map(|_| Token::SEMICOLON)
    ) - space().opt()
}

pub fn lexer<'a>() -> Parser<'a, u8, Vec<Token>> {
    ( lex_token().repeat(0..) + end() )
        .map(|(mut tokens, _eof)| {tokens.push(Token::EOF); tokens})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_tokens() {
        let input = "=+(){},;";
        let tokens = lexer().parse(input.as_bytes());

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
        let tokens = lexer().parse(input.as_bytes());

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
        let tokens = lexer().parse(input.as_bytes());

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
        let tokens = lexer().parse(input.as_bytes());

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
        let tokens = lexer().parse(input.as_bytes());

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
        let tokens = lexer().parse(input.as_bytes());

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
        let tokens = lexer().parse(input.as_bytes());

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
        let tokens = lexer().parse(input.as_bytes());

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
        let tokens = lexer().parse(input.as_bytes());

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

}
