use std::io::Write;
use super::{{Command, ShellStatus, CommandRegistry}};

pub struct HistoryCommand;

impl Command for HistoryCommand {
    fn execute(&self, args: &[String], registry: &CommandRegistry, _output: &mut dyn Write) -> Result<ShellStatus, String> {
        let history = registry.get_history();
        let mut i = 0;
        let limit = if args.len() > 0 {
            match args[0].parse::<usize>() {
                Ok(n) => n,
                Err(_) => return Err("history: invalid argument".to_string()),
            }
        } else {
            history.len()
        };

        let start = if limit > history.len() {
            return Err("history: argument greater than history size".to_string());
        } else {
            history.len() - limit
        };
        

        for (index, entry) in history.iter().enumerate().skip(start) {
            i += 1;
            if i > limit {
                break;
            }
            writeln!(_output, "    {}  {}", index + 1, entry).map_err(|e| e.to_string())?;
        }
        Ok(ShellStatus::Continue)
    }

    fn get_name(&self) -> &str {
        "history"
    }
}