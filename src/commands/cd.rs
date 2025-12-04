use super::{Command, ShellStatus, CommandRegistry};
use std::env;
use std::path::Path;

pub struct CdCommand;

impl Command for CdCommand {
    fn execute(&self, args: &[&str], _: &CommandRegistry) -> Result<ShellStatus, String> {
        if args.is_empty() {
             return Ok(ShellStatus::Continue);
        }

        let new_dir = args[0];
        let root = Path::new(new_dir);

        if let Err(_) = env::set_current_dir(&root) {
            return Err(format!("cd: {}: No such file or directory", new_dir));
        }

        Ok(ShellStatus::Continue)
    }

    fn get_name(&self) -> &str {
        "cd"
    }
}