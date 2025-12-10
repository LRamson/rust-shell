use std::io::Write;
use super::{{Command, ShellStatus, CommandRegistry}};

pub struct HistoryCommand;

impl Command for HistoryCommand {
    fn execute(&self, _args: &[String], registry: &CommandRegistry, _output: &mut dyn Write) -> Result<ShellStatus, String> {
        let history = registry.get_history();
        for (index, entry) in history.iter().enumerate() {
            writeln!(_output, "    {}  {}", index + 1, entry).map_err(|e| e.to_string())?;
        }
        Ok(ShellStatus::Continue)
    }

    fn get_name(&self) -> &str {
        "history"
    }
}