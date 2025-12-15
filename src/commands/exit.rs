use std::{env, io::Write};

use super::{{Command, ShellStatus, CommandRegistry}};

pub struct ExitCommand;

impl Command for ExitCommand {
    fn execute(&self, _: &[String], registry: &CommandRegistry, _: &mut dyn Write) -> Result<ShellStatus, String> {
        let path_hist = env::var("HISTFILE").unwrap_or_default();
        if path_hist != "" {
            let _ = registry.write_history_to_file(&path_hist, true);
        }
        Ok(ShellStatus::Exit)
    }

    fn get_name(&self) -> &str {
        "exit"
    }
}