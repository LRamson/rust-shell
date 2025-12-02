use std::{env, fs, os::unix::fs::PermissionsExt};
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut usr_input = String::new();
        io::stdin().read_line(&mut usr_input).unwrap();

        let trimmed_input = usr_input.trim();
        if trimmed_input.is_empty() {
            continue; 
        }

        let parts: Vec<&str> = trimmed_input.split_whitespace().collect();
        let command = parts[0];
        let args = &parts[1..];

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
                        let executable_path = get_executable_path(command_to_check);
                        if executable_path.is_some() {
                            println!("{} is {}", command_to_check, executable_path.unwrap());
                        } else {
                            println!("{}: not found", command_to_check);
                        }
                    }
                }
            }

            _ => {
                if let Some(executable_path) = get_executable_path(command) {
                    let status = std::process::Command::new(executable_path)
                        .args(args)
                        .status();
                } else {
                    println!("{}: command not found", command);
                }
            }
        }
    }
}

fn get_executable_path(command: &str) -> Option<String> {
    let path_var = env::var("PATH").unwrap_or_default();

    let paths = path_var.split(':');

    for path in paths {
        let full_path = format!("{}/{}", path, command);
        if fs::metadata(&full_path).map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false) {
            return Some(full_path);
        }
    }

    return None;
}
