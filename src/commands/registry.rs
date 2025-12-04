use super::{Command, ShellStatus}; 
use super::{echo::EchoCommand, exit::ExitCommand, type_cmd::TypeCommand, pwd::PwdCommand};
use std::collections::HashMap;
use std::os::unix::fs::PermissionsExt;
use std::{env, fs};

pub struct CommandRegistry {
    map: HashMap<String, Box<dyn Command>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        CommandRegistry {
            map: HashMap::new(),
        }
    }

    pub fn register(&mut self, command: Box<dyn Command>) {
        self.map.insert(command.get_name().to_string(), command);
    }

    pub fn get_command(&self, name: &str) -> Option<&Box<dyn Command>> {
        self.map.get(name)
    }

    pub fn run(&self, command_name: &str, args: &[&str]) -> Result<ShellStatus, String> {
        if let Some(command) = self.get_command(command_name) {
            command.execute(args, self)
        } else {
            self.run_external(command_name, args)
        }
    }

    pub fn run_external(&self, command_name: &str, args: &[&str]) -> Result<ShellStatus, String> {
        if let Some(_) = self.get_executable_path(command_name) {
            let status = std::process::Command::new(command_name)
                .args(args)
                .status()
                .map_err(|e| format!("Failed to execute {}: {}", command_name, e))?;
            
            if status.success() {
                Ok(ShellStatus::Continue)
            } else {
                Err(format!("{} exited with status {}", command_name, status))
            }
        }
        else {
            Err(format!("{}: command not found", command_name))
        }
    }

    pub fn get_executable_path(&self, command: &str) -> Option<String> {
        let path_var = env::var("PATH").unwrap_or_default();

        let paths = path_var.split(':');

        for path in paths {
            let full_path = format!("{}/{}", path, command);
            if fs::metadata(&full_path).map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false) {
                return Some(full_path);
            }
        }

        return None;
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        let mut registry = CommandRegistry::new();
        registry.register(Box::new(TypeCommand));
        registry.register(Box::new(EchoCommand));
        registry.register(Box::new(ExitCommand));
        registry.register(Box::new(PwdCommand));
        registry
    }
}