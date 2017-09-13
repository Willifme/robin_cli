extern crate rustyline;
extern crate robin_core;

use rustyline::Editor;
use robin_core::parser::number;

fn main() {
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline(">>> ");

        match readline {
            Ok(line) => {
                println!("{:?}", number::numeric_literals(line.as_bytes()));

                rl.add_history_entry(&line);
            },

            Err(_) => break, 
        }
    }
}
