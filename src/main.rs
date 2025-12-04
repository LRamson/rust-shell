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

        let parts = utils::parse_input(&input);
        
        if parts.is_empty() {
            continue;
        }

        let command = parts[0];
        let args = &parts[1..];

        match registry.run(command, args) {
            Ok(ShellStatus::Exit) => break,
            Ok(ShellStatus::Continue) => continue,
            Err(e) => {
                println!("{}", e); 
            }
        }
    }
}