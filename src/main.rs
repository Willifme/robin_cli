extern crate rustyline;
extern crate robin_core;

use rustyline::Editor;
use robin_core::parser;
use robin_core::error;

struct Repl {
    editor: Editor<()>,
    error_stack: error::ErrorStack,
}

impl Repl {
    fn new() -> Repl {
        Repl {
            editor: Editor::<()>::new(),
            error_stack: error::ErrorStack(vec![]),
        }
    }

    fn repl(&mut self) {
        loop {
            let readline = self.editor.readline(">>> ");

            match readline {
                Ok(line) => {
                    self.parse(line.clone());

                    self.editor.add_history_entry(&line);
                }

                Err(_) => break, 
            }
        }
    }

    fn parse(&mut self, line: String) {
        match parser::parse(line) {
            Ok(expr) => println!("{:?}", expr),
            Err(e) => {
                // TODO: Remove this unwrap
                self.error_stack
                    .0
                    .push(error::Error((error::ErrorLevel::Error, e)));

                // TODO: Remove unwrap
                println!("{}", self.error_stack.0.last().unwrap());
            }
        }
    }
}
fn main() {
    let mut repl = Repl::new();

    repl.repl();
}
