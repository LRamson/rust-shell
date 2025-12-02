#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let mut usr_input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut usr_input).unwrap();

        let parts: Vec<&str> = usr_input.trim().split_whitespace().collect();
        let command = parts[0];
        let args = &parts[1..];

        if command == "exit" {
            break;
        } else if command.trim() == "echo" {
            println!("{}", args.join(" "));
            continue;
        }

        println!("{}: command not found", command.trim())
    }
}
