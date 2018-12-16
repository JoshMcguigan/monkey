mod lexer;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                println!("{:?}", lexer::lexer().parse(line.as_bytes()));
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
