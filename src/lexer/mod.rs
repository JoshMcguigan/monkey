use pom::Parser;
use pom::parser::{seq, one_of, is_a, not_a, end};

use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Token {
//    ILLEGAL, // this variant may not be necessary given the way I am lexing
    EOF,
    IDENT(String),
    INT(u32),
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}

fn space() -> Parser<u8, ()> {
    one_of(b" \t\r\n").repeat(1..).discard()
}

fn lex_let() -> Parser<u8, Token> {
    seq(b"let").map(|_| Token::LET) - space()
}

fn lex_fn() -> Parser<u8, Token> {
    seq(b"fn").map(|_| Token::FUNCTION)
        - -not_a(|byte| char::from(byte).is_alphabetic())
}

fn lex_ident() -> Parser<u8, Token> {
    is_a(|byte| char::from(byte).is_alphabetic())
        .repeat(1..).convert(String::from_utf8)
        .map(|ident| Token::IDENT(ident) )
}

fn lex_int() -> Parser<u8, Token> {
    is_a(|byte| char::from(byte).is_numeric())
        .repeat(1..)
        .convert(String::from_utf8)
        .convert(|num|u32::from_str(&num))
        .map(|num| Token::INT(num) )
}

fn lex_token() -> Parser<u8, Token> {
    space().opt() * (
          lex_let()
        | lex_fn()
        | lex_ident()
        | lex_int()
        | seq(b"=").map(|_| Token::ASSIGN)
        | seq(b"+").map(|_| Token::PLUS)
        | seq(b"(").map(|_| Token::LPAREN)
        | seq(b")").map(|_| Token::RPAREN)
        | seq(b"{").map(|_| Token::LBRACE)
        | seq(b"}").map(|_| Token::RBRACE)
        | seq(b",").map(|_| Token::COMMA)
        | seq(b";").map(|_| Token::SEMICOLON)
    ) - space().opt()
}

fn lex() -> Parser<u8, Vec<Token>> {
    ( lex_token().repeat(0..) + end() )
        .map(|(mut tokens, _eof)| {tokens.push(Token::EOF); tokens})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_tokens() {
        let input = "=+(){},;";
        let tokens = lex().parse(input.as_bytes());

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
        let tokens = lex().parse(input.as_bytes());

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
        let tokens = lex().parse(input.as_bytes());

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
        let tokens = lex().parse(input.as_bytes());

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
        let tokens = lex().parse(input.as_bytes());

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
        let tokens = lex().parse(input.as_bytes());

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
}
