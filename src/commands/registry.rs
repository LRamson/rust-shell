use super::{Command}; 
use super::{echo::EchoCommand, exit::ExitCommand, type_cmd::TypeCommand, pwd::PwdCommand, cd::CdCommand, history::HistoryCommand};
use std::cell::RefCell;
use std::collections::HashMap;
use std::os::unix::fs::PermissionsExt;
use std::{env, fs};

pub struct CommandRegistry {
    pub builtins: HashMap<String, Box<dyn Command>>,
    pub executables: HashMap<String, String>,

    history: RefCell<Vec<String>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        CommandRegistry {
            builtins: HashMap::new(),
            executables: HashMap::new(),

            history: RefCell::new(Vec::new()),
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

    fn register_builtin(&mut self, command: Box<dyn Command>) {
        self.builtins.insert(command.get_name().to_string(), command);
    }


    fn register_executable(&mut self, name: &str, path: &str) {
        self.executables.insert(name.to_string(), path.to_string());
    }

    pub fn add_history_entry(&self, cmd: &str) {
        self.history.borrow_mut().push(cmd.to_string());
    }

    pub fn get_history(&self) -> Vec<String> {
        self.history.borrow().clone()
    }

    pub fn load_history_from_file(&self, path: &str) -> Result<(), String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let mut lines: Vec<String> = content.lines().map(|line| line.to_string()).collect();
        self.history.borrow_mut().append(&mut lines);
        Ok(())
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
        registry.register_builtin(Box::new(TypeCommand));
        registry.register_builtin(Box::new(EchoCommand));
        registry.register_builtin(Box::new(ExitCommand));
        registry.register_builtin(Box::new(PwdCommand));
        registry.register_builtin(Box::new(CdCommand));
        registry.register_builtin(Box::new(HistoryCommand));

        registry.scan_path_executables();

        registry
    }
}