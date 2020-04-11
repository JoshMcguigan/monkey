mod parser;
use crate::parser::parse;

mod lexer;
use crate::lexer::lex;

mod eval;
use crate::eval::{eval_return_scope, Object, Env};

mod code;
mod compiler;
mod vm;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new();
    let mut env = Env::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let mut tokens = lex(&line);
                let ast = parse(&mut tokens);
                display_object(eval_return_scope(ast, &mut env));
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
        Object::String(string) => println!("{}", string),
        Object::Boolean(val) => println!("{}", val),
        Object::Function{parameters: _, body: _} => println!("function"),
        Object::Null => println!("null"),
        Object::Return(obj) => display_object(*obj),
    }
}
