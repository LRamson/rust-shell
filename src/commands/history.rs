use super::{Command, ShellStatus, CommandRegistry};
use std::{io::Write};

pub struct HistoryCommand;

impl Command for HistoryCommand {
    fn execute(&self, args: &[String], registry: &CommandRegistry, output: &mut dyn Write) -> Result<ShellStatus, String> {
        let history = registry.get_history();
        
        let limit = match args.first() {
            Some(arg) => {
                if arg == "-r" {
                    match args.get(1) {
                        Some(path_arg) => registry.load_history_from_file(path_arg)?,
                        None => return Err("history: -r requires a path argument".to_string()),
                    }
                    return Ok(ShellStatus::Continue);
                }
                
                arg.parse::<usize>()
                .map_err(|_| format!("history: {}: numeric argument required", arg))?
            },
            None => history.len(), 
        };

        let start_index = history.len().saturating_sub(limit);

        for (i, entry) in history.iter().enumerate().skip(start_index) {
            writeln!(output, "{:>5}  {}", i + 1, entry).map_err(|e| e.to_string())?;
        }

        Ok(ShellStatus::Continue)
    }

    fn get_name(&self) -> &str {
        "history"
    }
}