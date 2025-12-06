use super::{Command, ShellStatus}; 
use super::{echo::EchoCommand, exit::ExitCommand, type_cmd::TypeCommand, pwd::PwdCommand, cd::CdCommand};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::Stdio;
use std::{env, fs};
use crate::utils::ParsedCommand;

pub struct CommandRegistry {
    builtins: HashMap<String, Box<dyn Command>>,
    executables: HashMap<String, String>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        CommandRegistry {
            builtins: HashMap::new(),
            executables: HashMap::new(),
        }
    }
    
    pub fn get_builtin(&self, name: &str) -> Option<&Box<dyn Command>> {
        self.builtins.get(name)
    }

    pub fn get_executable(&self, name: &str) -> Option<&String> {
        self.executables.get(name)
    }
    
    pub fn get_command_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.builtins.keys().cloned().collect();
        names.extend(self.executables.keys().cloned());
        
        names.sort();
        names.dedup();
        
        names
    }
    
    pub fn run(&self, parsed: &ParsedCommand) -> Result<ShellStatus, String> {
        if let Some(cmd) = self.get_builtin(&parsed.command) {
            self.run_builtin(cmd, &parsed.args,
                 &parsed.stdout_redirect, &parsed.stderr_redirect,
                 parsed.stdout_redirect_append, parsed.stderr_redirect_append)
        } else {
            self.run_external(&parsed.command, &parsed.args,
                &parsed.stdout_redirect, &parsed.stderr_redirect,
                 parsed.stdout_redirect_append, parsed.stderr_redirect_append)
        }
    }
    
    fn run_builtin(&self, cmd: &Box<dyn Command>, args: &[String],
        output_path: &Option<String>, err_path: &Option<String>, append: bool, append_err: bool) -> Result<ShellStatus, String> {
            let mut writer_output: Box<dyn Write> = match output_path {
            Some(path) => {
                let file = self.get_output_file(path, append).map_err(|e| format!("Failed to open {}: {}", path, e))?;
                Box::new(file)
            }
            None => {
                Box::new(io::stdout())
            }
        };

        let mut writer_err: Box<dyn Write> = match err_path {
            Some(path) => {
                let file = self.get_output_file(path, append_err)
                .map_err(|e| format!("Failed to open {}: {}", path, e))?;
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


    fn register_builtin(&mut self, command: Box<dyn Command>) {
        self.builtins.insert(command.get_name().to_string(), command);
    }


    fn register_executable(&mut self, name: &str, path: &str) {
        self.executables.insert(name.to_string(), path.to_string());
    }


    fn scan_path_executables(&mut self) {
        let path_var = env::var("PATH").unwrap_or_default();

        let paths = path_var.split(':');

        for path in paths {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_name = entry.file_name().into_string().unwrap_or_default();
                        let full_path = format!("{}/{}", path, file_name);
                        if fs::metadata(&full_path).map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false) {
                            self.register_executable(&file_name, &full_path);
                        }
                    }
                }
            }
        }
    }
    

    pub fn run_external(&self, command_name: &str, args: &[String], 
        output_path: &Option<String>, err_path: &Option<String>, append: bool, append_err: bool) -> Result<ShellStatus, String> {
        if let Some(_) = self.get_executable(command_name) {
            let stdout_dest = self.get_output_stdio(output_path.as_deref(), append);

            let err_dest = self.get_output_stdio(err_path.as_deref(), append_err);

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

    fn get_output_stdio(&self, path: Option<&str>, append: bool) -> Stdio {
        match path {
                Some(path) => {
                    let file = self.get_output_file(path, append)
                        .map_err(|e| format!("Failed to open {}: {}", path, e));
                    match file {
                        Ok(f) => Stdio::from(f),
                        Err(_) => Stdio::inherit(),
                    }
                }
                None => Stdio::inherit(), 
            }
    }

    fn get_output_file(&self, path: &str, append: bool) -> io::Result<File> {
        OpenOptions::new()
            .create(true) 
            .write(true)  
            .truncate(!append) 
            .append(append)    
            .open(path)
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        let mut registry = CommandRegistry::new();
        registry.register_builtin(Box::new(TypeCommand));
        registry.register_builtin(Box::new(EchoCommand));
        registry.register_builtin(Box::new(ExitCommand));
        registry.register_builtin(Box::new(PwdCommand));
        registry.register_builtin(Box::new(CdCommand));

        registry.scan_path_executables();

        registry
    }
}