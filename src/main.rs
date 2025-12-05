mod commands; 
mod utils;    

use std::io::{self, Write};
use commands::{CommandRegistry, ShellStatus};

fn main() {
    let registry = CommandRegistry::default();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parsed_cmd = match utils::parse_command_line(&input) {
            Some(cmd) => cmd,
            None => continue, 
        };

        match registry.run(&parsed_cmd) {
            Ok(ShellStatus::Exit) => break,
            Ok(ShellStatus::Continue) => continue,
            Err(e) => eprintln!("{}", e), 
        }
    }
}