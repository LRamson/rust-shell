#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut usr_input = String::new();
        io::stdin().read_line(&mut usr_input).unwrap();

        // 1. SAFETY FIX: Handle empty input (just hitting Enter)
        let trimmed_input = usr_input.trim();
        if trimmed_input.is_empty() {
            continue; // Skip the rest of the loop and show "$ " again
        }

        let parts: Vec<&str> = trimmed_input.split_whitespace().collect();
        let command = parts[0];
        let args = &parts[1..];

        // 2. ORGANIZATION: Use 'match' instead of if/else
        match command {
            "exit" => break,
            "echo" => {
                println!("{}", args.join(" "));
            }
            "type" => {
                // We need to check if the user actually provided an argument
                // e.g. "type echo" vs just "type"
                if args.is_empty() {
                    continue; 
                }
                
                let command_to_check = args[0];

                // Check against our known builtins
                match command_to_check {
                    "echo" | "exit" | "type" => {
                        println!("{} is a shell builtin", command_to_check);
                    }
                    _ => {
                        println!("{}: not found", command_to_check);
                    }
                }
            }
            // 3. DEFAULT CASE: Unknown command
            _ => {
                println!("{}: command not found", command);
            }
        }
    }
}