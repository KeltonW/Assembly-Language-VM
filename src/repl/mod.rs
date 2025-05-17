use crate::repl::std::num::ParseIntError;
use crate::vm::VM;
use std;
use std::io;
use std::io::Write;
// core struct for repl assembler
pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM, // vm that repl will use to execute code
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
                ".program" => {
                    println!("Listing instructions currently in vm's program vector:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("End of program listing");
                }
                ".registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of register listing");
                }
                _ => {
                    let results = self.parse_hex(buffer);
                    match results {
                        Ok(bytes) => {
                            for byte in bytes {
                                self.vm.add_byte(byte);
                            }
                        }
                        Err(_e) => {
                            println!(
                                "Unable to parse hex string. Please enter 4 groups of to hex digits."
                            );
                        }
                    };
                    self.vm.run_once();
                }
            }
        }
    }
    // parse a string of hex values without '0x' and return a Vec of u8
    pub fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }
}
