extern crate rustyline;
extern crate robin_core;

use std::process::{Command, Child, Stdio};
use std::io::{Read, Write};

use rustyline::Editor;
use robin_core::parser;
use robin_core::compiler::Compiler;

struct Repl {
    editor: Editor<()>,
    compiler: Compiler,
    node: Child,
}

impl Repl {
    fn new() -> Repl {
        Repl {
            editor: Editor::<()>::new(),
            compiler: Compiler::new(),
            node: Command::new("node") 
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())

                        // Use the --interactive flag to start the REPL despite stdin not
                        // being a terminal see node --help for more
                        .args(&["-i"])
                        .spawn()
                        .expect("Node.js not installed or found in path, please fix this!")
        }
    }

    fn repl(&mut self) {
        println!("Welcome to Robin! Type code in below.");

        println!("When you run code in the REPL, you might find some Robin and some Node.js errors! Be careful");

        loop {
            let readline = self.editor.readline(">>> ");

            match readline {
                Ok(line) => {
                    self.parse(&line);

                    self.editor.add_history_entry(&line);
                }

                Err(_) => break, 
            }
        }
    }

    fn handle_node_repl(&mut self, input: String) {
        if let Some(ref mut stdin) = self.node.stdin.as_mut() {
            write!(stdin, "{}\n", input);

            stdin.flush();
        }

        self.node.wait().expect("Failed to read stdout");

        let mut buffer = String::new();

        if let Some(ref mut stdout) = self.node.stdout {
            stdout.read_to_string(&mut buffer).unwrap();

            println!("{}", buffer);
        }
    }

    fn parse(&mut self, line: &str) {
        match parser::parse(line) {
            // TODO: Remove this unwrap
            Ok(expr) => { 
                let output = self.compiler.compile(&[expr]);

                // TODO: Handle error stack here
                if !self.compiler.errors.0.is_empty() {
                    self.compiler.errors.0
                        .iter()
                        .for_each(|e| println!("{}", e));

                    // Clear the error stack on each input
                    self.compiler.errors.0.clear();

                } else {
                    println!("JS Code: {}", output);

                    self.handle_node_repl(output);
                }
            },

            Err(e) => {
                // TODO: Handle this error
                println!("Error");
            }
        }
    }
}

fn main() {
    let mut repl = Repl::new();

    repl.repl();
}
