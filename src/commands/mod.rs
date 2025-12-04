mod echo;
mod exit;
mod type_cmd;
mod registry;
mod pwd;

pub use registry::CommandRegistry;

pub enum ShellStatus {
    Continue,
    Exit,
}

pub trait Command {
    fn execute(&self, args: &[&str], registry: &CommandRegistry) -> Result<ShellStatus, String>;
    fn get_name(&self) -> &str;
    fn get_type(&self) -> &str {
        "shell builtin"
    }
}

