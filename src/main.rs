#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let mut command = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).unwrap();

        if command.trim() == "exit" {
            break;
        }
        
        println!("{}: command not found", command.trim())
    }
}
