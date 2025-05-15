use crate::vm::VM;
use std;
use std::io;
use std::io::Write;

// core struct for repl assembler
pub struct REPL {
    pub command_buffer: Vec<String>,
    pub vm: VM, // vm that repl will use to execute code
}
impl REPL {
    // create and return new assembly repl
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
        }
    }
    pub fn run(&mut self) {
        println!("Welcome to Iridium!");
        loop {
            // allocate new String in which to store user input
            //TODO: create this outside of the loop and store it each iteration
            let mut buffer = String::new();

            // block call until user inputs a command
            let stdin = io::stdin();

            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            // evaluate user input
            stdin.read_line(&mut buffer).expect("Unable to read line");
            let buffer = buffer.trim();

            // store a copy of each command
            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".quit" => {
                    println!("Goodbye!");
                    std::process::exit(0);
                }
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                _ => {
                    println!("Invalid input");
                }
            }
        }
    }
}
