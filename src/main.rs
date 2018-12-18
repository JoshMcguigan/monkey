mod parser;
use crate::parser::parse;

mod lexer;
use crate::lexer::lexer;

mod eval;
use crate::eval::{eval_statements, Object};

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let mut tokens = lexer().parse(line.as_bytes()).unwrap();
                let ast = parse(&mut tokens);
                display_object(eval_statements(ast));
            },
            Err(ReadlineError::Interrupted) => {
                break
            },
            Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break
            }
        }
    }
}

fn display_object(obj: Object) {
    match obj {
        Object::Integer(num) => println!("{}", num),
        Object::Boolean(val) => println!("{}", val),
        Object::Null => println!("null"),
        Object::Return(obj) => display_object(*obj),
    }
}
