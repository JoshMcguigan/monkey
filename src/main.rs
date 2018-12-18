mod parser;
use crate::parser::parse;

mod lexer;
use crate::lexer::lexer;

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
                println!("{:#?}", ast);
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
