use super::{{Command, ShellStatus, CommandRegistry}};

pub struct EchoCommand;

impl Command for EchoCommand {
    fn execute(&self, args: &[String], _: &CommandRegistry) -> Result<ShellStatus, String> {
        println!("{}", args.join(" "));
        Ok(ShellStatus::Continue)
    }

    fn get_name(&self) -> &str {
        "echo"
    }
}