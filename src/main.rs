mod commands; 
mod utils;    
mod ui;

use std::io::{self, Write};
use commands::{CommandRegistry, ShellStatus};
use ui::ShellHelper;
use rustyline::Editor;

fn main() {
    let registry = CommandRegistry::default();

    let command_names = registry.get_command_names();

    let helper = ShellHelper::new(command_names);

    let mut editor = Editor::<ShellHelper, _>::new().unwrap();
    editor.set_helper(Some(helper));

    loop {
        let readline = editor.readline("$ ");

        match readline {
            Ok(line) => {
                editor.add_history_entry(line.as_str()).ok();
                let parsed_cmd = match utils::parse_command_line(&line) {
                    Some(cmd) => cmd,
                    None => continue, 
                };
                
                match registry.run(&parsed_cmd) {
                    Ok(ShellStatus::Exit) => break,
                    Ok(ShellStatus::Continue) => continue,
                    Err(e) => eprintln!("{}", e), 
                }
            }
            Err(_) => {
                break;
            }
        }
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

    }
}