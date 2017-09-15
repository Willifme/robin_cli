extern crate rustyline;
extern crate robin_core;

use rustyline::Editor;
use robin_core::parser::expression::{ParseResult, parse_expression};
use std::str;

fn main() {
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline(">>> ");

        match readline {
            Ok(line) => {
                match parse_expression(line.as_bytes()) {
                    ParseResult::Done(expr) => println!("{:?}", expr),
                    ParseResult::Error(err, ref expr) => {
                        // TODO: Remove this unwrap
                        println!("Error: {}, Expression: {:?}",
                                 str::from_utf8(err).unwrap(),
                                 &expr);
                    }
                }

                rl.add_history_entry(&line);
            }

            Err(_) => break,
        }
    }
}
