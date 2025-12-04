use super::{Command, ShellStatus};
use super::CommandRegistry;

pub struct TypeCommand;

impl Command for TypeCommand {
    fn execute(&self, args: &[String], registry: &CommandRegistry) -> Result<ShellStatus, String> {
        if args.is_empty()  {
            return Err("type: missing argument".to_string());
        }

        for arg in args {
            if let Some(command) = registry.get_command(&arg) {
                println!("{} is a {}", arg, command.get_type());
            } else if let Some(executable_path) = registry.get_executable_path(&arg) {
                println!("{} is {}", arg, executable_path);
            } else {
                println!("{}: not found", arg);
            }
        }

        Ok(ShellStatus::Continue)
    }

    fn get_name(&self) -> &str {
        "type"
    }
}