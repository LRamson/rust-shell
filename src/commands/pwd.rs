use super::{{Command, ShellStatus, CommandRegistry}};

pub struct PwdCommand;

impl Command for PwdCommand {
    fn execute(&self, _: &[String], _: &CommandRegistry) -> Result<ShellStatus, String> {
        let current_dir = std::env::current_dir()
            .map_err(|e| format!("pwd: failed to get current directory: {}", e))?;
        println!("{}", current_dir.display());
        Ok(ShellStatus::Continue)
    }

    fn get_name(&self) -> &str {
        "pwd"
    }
}