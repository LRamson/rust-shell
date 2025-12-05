use std::io::Write;

use super::{{Command, ShellStatus, CommandRegistry}};

pub struct PwdCommand;

impl Command for PwdCommand {
    fn execute(&self, _: &[String], _: &CommandRegistry, output: &mut dyn Write) -> Result<ShellStatus, String> {
        let current_dir = std::env::current_dir()
            .map_err(|e| format!("pwd: failed to get current directory: {}", e))?;
        writeln!(output, "{}", current_dir.display()).map_err(|e| e.to_string())?;
        Ok(ShellStatus::Continue)
    }

    fn get_name(&self) -> &str {
        "pwd"
    }
}