use std::io::Write;

use super::{{Command, ShellStatus, CommandRegistry}};

pub struct EchoCommand;

impl Command for EchoCommand {
    fn execute(&self, args: &[String], _: &CommandRegistry, output: &mut dyn Write) -> Result<ShellStatus, String> {
        writeln!(output, "{}", args.join(" ")).map_err(|e| e.to_string())?;
        Ok(ShellStatus::Continue)
    }

    fn get_name(&self) -> &str {
        "echo"
    }
}