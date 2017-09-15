extern crate rustyline;
extern crate robin_core;

use rustyline::Editor;
use robin_core::parser::expression;

fn main() {
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline(">>> ");

        match readline {
            Ok(line) => {
                println!("{:?}", expression::expression_literal(line.as_bytes()));

                rl.add_history_entry(&line);
            },

            Err(_) => break, 
        }
    }
}
