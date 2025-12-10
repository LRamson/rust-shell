mod commands; 
mod utils;    
mod ui;

use commands::{CommandRegistry, ShellStatus, ShellExecutor};
use ui::ShellHelper;
use rustyline::{CompletionType, Config, EditMode, Editor, error::ReadlineError};

fn main() {
    let registry = CommandRegistry::default();
    let command_names = registry.get_command_names();
    let helper = ShellHelper::new(command_names);
    let executor = ShellExecutor::new(&registry);

    let config = Config::builder()
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs) 
        .build();

    let mut editor = 
        Editor::<ShellHelper, _>::with_config(config).unwrap();
    editor.set_helper(Some(helper));

    loop {
        let readline = editor.readline("$ ");
        match readline {
            Ok(line) => {
                registry.add_history_entry(&line);
                // editor.add_history_entry(line.as_str()).ok();

                let commands = utils::parse_input(line.as_str());
                
                if commands.is_empty() {
                    continue;
                }

                match executor.run(&commands) {
                    Ok(ShellStatus::Exit) => break,
                    Ok(ShellStatus::Continue) => continue,
                    Err(e) => eprintln!("{}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("exit");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}