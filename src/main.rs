extern crate rustyline;
extern crate robin_core;

use rustyline::Editor;
use robin_core::parser::expression::{ParseResult, parse_expression};
use robin_core::error;

struct Repl {
    editor: Editor<()>,
    error_stack: error::ErrorStack,
}

impl Repl {
    fn new() -> Repl {
        Repl { editor: Editor::<()>::new(), error_stack: vec![] }
    }

    fn repl(&mut self) {
        loop {
            let readline = self.editor.readline(">>> ");

            match readline {
                Ok(line) => {
                    match parse_expression(line.as_bytes()) {
                        ParseResult::Done(expr) => println!("{:?}", expr),
                        ParseResult::Error(_, _) => {
                            // TODO: Remove this unwrap
                            self.error_stack.push((error::ErrorLevel::Error, "Parse error"));
                        }
                    }

                    self.editor.add_history_entry(&line);
                }

                Err(_) => break,
            }
        }
    }
}
fn main() {
    let mut repl = Repl::new();

    repl.repl();

    // TODO: Implement display for the error stack
    if !repl.error_stack.is_empty() {
        for error in repl.error_stack {
            println!("{:?}", error);
        }
    }
}
