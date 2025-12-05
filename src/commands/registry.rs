use super::{Command, ShellStatus}; 
use super::{echo::EchoCommand, exit::ExitCommand, type_cmd::TypeCommand, pwd::PwdCommand, cd::CdCommand};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::Stdio;
use std::{env, fs};
use crate::utils::ParsedCommand;

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

    pub fn run(&self, parsed: &ParsedCommand) -> Result<ShellStatus, String> {
        if let Some(cmd) = self.get_command(&parsed.command) {
            self.run_builtin(cmd, &parsed.args, &parsed.stdout_redirect, &parsed.stderr_redirect)
        } else {
            self.run_external(&parsed.command, &parsed.args, &parsed.stdout_redirect, &parsed.stderr_redirect)
        }
    }

    fn run_builtin(&self, cmd: &Box<dyn Command>, args: &[String],
         output_path: &Option<String>, err_path: &Option<String>) -> Result<ShellStatus, String> {
        let mut writer_output: Box<dyn Write> = match output_path {
            Some(path) => {
                let file = File::create(path).map_err(|e| format!("Failed to open {}: {}", path, e))?;
                Box::new(file)
            }
            None => {
                Box::new(io::stdout())
            }
        };

        let mut writer_err: Box<dyn Write> = match err_path {
            Some(path) => {
                let file = File::create(path).map_err(|e| format!("Failed to open {}: {}", path, e))?;
                Box::new(file)
            }
            None => {
                Box::new(io::stderr())
            }
        };

        match cmd.execute(args, self, &mut *writer_output) {
            Ok(status) => Ok(status),
            Err(e) => {
                writeln!(writer_err, "{}", e).map_err(|e| e.to_string())?;
                Ok(ShellStatus::Continue)
            }
        }
    }

    pub fn run_external(&self, command_name: &str, args: &[String], 
         output_path: &Option<String>, err_path: &Option<String>) -> Result<ShellStatus, String> {
        if let Some(_) = self.get_executable_path(command_name) {
            let stdout_dest = match output_path {
                Some(path) => {
                    let file = File::create(path).map_err(|e| format!("Failed to open {}: {}", path, e))?;
                    Stdio::from(file) 
                }
                None => Stdio::inherit(), 
            };

            let err_dest = match err_path {
                Some(path) => {
                    let file = File::create(path).map_err(|e| format!("Failed to open {}: {}", path, e))?;
                    Stdio::from(file) 
                }
                None => Stdio::inherit(), 
            };

            let _ = std::process::Command::new(command_name)
                .args(args)
                .stdout(stdout_dest)
                .stderr(err_dest)
                .status()
                .map_err(|e| format!("Failed to execute {}: {}", command_name, e))?;
            
            Ok(ShellStatus::Continue)
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
        registry.register(Box::new(CdCommand));
        registry
    }
}