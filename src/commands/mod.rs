use std::io::Write;

mod echo;
mod exit;
mod type_cmd;
mod registry;
mod pwd;
mod cd;
mod executor;

pub use registry::CommandRegistry;
pub use executor::ShellExecutor;

pub enum ShellStatus {
    Continue,
    Exit,
}

pub trait Command {
    fn execute(&self, args: &[String], registry: &CommandRegistry,
         output: &mut dyn Write) -> Result<ShellStatus, String>;
    fn get_name(&self) -> &str;
    fn get_type(&self) -> &str {
        "shell builtin"
    }
}

