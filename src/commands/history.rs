use std::io::Write;

use super::{{Command, ShellStatus, CommandRegistry}};

pub struct HistoryCommand;

impl Command for HistoryCommand {
    fn execute(&self, _args: &[String], _: &CommandRegistry, _output: &mut dyn Write) -> Result<ShellStatus, String> {
        Ok(ShellStatus::Continue)
    }

    fn get_name(&self) -> &str {
        "history"
    }
}