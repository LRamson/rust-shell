use std::io::Write;

use super::{Command, ShellStatus};
use super::CommandRegistry;

pub struct TypeCommand;

impl Command for TypeCommand {
    fn execute(&self, args: &[String], registry: &CommandRegistry, output: &mut dyn Write) -> Result<ShellStatus, String> {
        if args.is_empty()  {
            return Err("type: missing argument".to_string());
        }

        for arg in args {
            if let Some(command) = registry.get_command(&arg) {
                writeln!(output, "{} is a {}", arg, command.get_type()).map_err(|e| e.to_string())?;
            } else if let Some(executable_path) = registry.get_executable_path(&arg) {
                writeln!(output, "{} is {}", arg, executable_path).map_err(|e| e.to_string())?;
            } else {
                writeln!(output, "{}: not found", arg).map_err(|e| e.to_string())?;
            }
        }

        Ok(ShellStatus::Continue)
    }

    fn get_name(&self) -> &str {
        "type"
    }
}