use super::{Command, ShellStatus, CommandRegistry};
use std::io::Write;

pub struct HistoryCommand;

impl Command for HistoryCommand {
    fn execute(&self, args: &[String], registry: &CommandRegistry, output: &mut dyn Write) -> Result<ShellStatus, String> {
        match args.first().map(|s| s.as_str()) {
            Some("-w") => {
                let path = args.get(1).ok_or("history: -w: argument required")?;
                registry.write_history_to_file(path, false, false)?;
                Ok(ShellStatus::Continue)
            },

            Some("-a") => {
                let path = args.get(1).ok_or("history: -a: argument required")?;
                registry.write_history_to_file(path, true, false)?;
                Ok(ShellStatus::Continue)
            },
            
            Some("-r") => {
                let path = args.get(1).ok_or("history: -r: argument required")?;
                registry.load_history_from_file(path)?;
                Ok(ShellStatus::Continue)
            },
        
            _ => self.list_history(args, registry, output),
        }
    }

    fn get_name(&self) -> &str {
        "history"
    }
}

impl HistoryCommand {
    fn list_history(&self, args: &[String], registry: &CommandRegistry, output: &mut dyn Write) -> Result<ShellStatus, String> {
        let history = registry.get_history();
        
        let limit = match args.first() {
            Some(arg) => arg.parse::<usize>()
                .map_err(|_| format!("history: {}: numeric argument required", arg))?,
            None => history.len(),
        };

        let start_index = history.len().saturating_sub(limit);

        for (i, entry) in history.iter().enumerate().skip(start_index) {
            writeln!(output, "{:>5}  {}", i + 1, entry).map_err(|e| e.to_string())?;
        }

        Ok(ShellStatus::Continue)
    }
}