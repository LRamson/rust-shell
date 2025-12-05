use std::io::Write;

use super::{{Command, ShellStatus, CommandRegistry}};

pub struct ExitCommand;

impl Command for ExitCommand {
    fn execute(&self, _: &[String], _: &CommandRegistry, _: &mut dyn Write) -> Result<ShellStatus, String> {
        Ok(ShellStatus::Exit)
    }

    fn get_name(&self) -> &str {
        "exit"
    }
}