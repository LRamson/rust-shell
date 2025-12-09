use std::process::{Command as ProcessCommand, Stdio, Child};
use std::fs::{File, OpenOptions};
use std::io::{self, Write}; 
use super::{CommandRegistry, ShellStatus};
use crate::utils::ParsedCommand;

enum PipeState {
    None,
    Process(Child),  
    Buffer(Vec<u8>),  
}

pub struct ShellExecutor<'a> {
    registry: &'a CommandRegistry,
}

impl<'a> ShellExecutor<'a> {
    pub fn new(registry: &'a CommandRegistry) -> Self {
        Self { registry }
    }

    pub fn run(&self, pipeline: &[ParsedCommand]) -> Result<ShellStatus, String> {
        if pipeline.is_empty() {
            return Ok(ShellStatus::Continue);
        }

        let mut previous_output = PipeState::None;
        let mut iter = pipeline.iter().peekable();

        while let Some(cmd) = iter.next() {
            let is_last = iter.peek().is_none();
            
            let is_builtin = self.registry.get_builtin(&cmd.command).is_some();

            let (new_state, status) = if is_builtin {
                self.handle_builtin(cmd, &mut previous_output, is_last)?
            } else {
                self.handle_external(cmd, &mut previous_output, is_last)?
            };

            if let ShellStatus::Exit = status {
                return Ok(ShellStatus::Exit);
            }

            previous_output = new_state;
        }

        if let PipeState::Process(mut child) = previous_output {
             child.wait().map_err(|e| e.to_string())?;
        }

        Ok(ShellStatus::Continue)
    }


    fn handle_builtin(
        &self, 
        cmd: &ParsedCommand, 
        _input: &mut PipeState, 
        is_last: bool
    ) -> Result<(PipeState, ShellStatus), String> {
        
        let builtin = self.registry.get_builtin(&cmd.command).unwrap();
        
        let mut output_buffer = Vec::new();
        let mut writer: Box<dyn Write> = if let Some(path) = &cmd.stdout_redirect {
             let file = self.open_file(path, cmd.stdout_redirect_append)?;
             Box::new(file)
        } else if !is_last {
             Box::new(&mut output_buffer)
        } else {
             Box::new(io::stdout())
        };

        let result = builtin.execute(&cmd.args, self.registry, &mut *writer);

        drop(writer); 

        match result {
            Ok(status) => {
                if !is_last && cmd.stdout_redirect.is_none() {
                    Ok((PipeState::Buffer(output_buffer), status))
                } else {
                    Ok((PipeState::None, status))
                }
            },
            Err(e) => {
                if let Some(path) = &cmd.stderr_redirect {
                    let mut file = self.open_file(path, cmd.stderr_redirect_append)?;
                    writeln!(file, "{}", e).map_err(|e| e.to_string())?;
                    Ok((PipeState::None, ShellStatus::Continue))
                } else {
                    Err(e)
                }
            }
        }
    }


    fn handle_external(
        &self,
        cmd: &ParsedCommand,
        input: &mut PipeState,
        is_last: bool
    ) -> Result<(PipeState, ShellStatus), String> {
        if self.registry.get_executable(&cmd.command).is_none() {
            return Err(format!("{}: command not found", cmd.command));
        }
        
        let stdin = match input {
            PipeState::Process(child) => {
                if let Some(out) = child.stdout.take() {
                    Stdio::from(out)
                } else {
                    Stdio::null()
                }
            },
            PipeState::Buffer(_) => {
                Stdio::piped() 
            },
            PipeState::None => Stdio::inherit(),
        };

        let (stdout, creates_pipe) = if let Some(path) = &cmd.stdout_redirect {
            let file = self.open_file(path, cmd.stdout_redirect_append)?;
            (Stdio::from(file), false)
        } else if !is_last {
            (Stdio::piped(), true)
        } else {
            (Stdio::inherit(), false)
        };

        let stderr = if let Some(path) = &cmd.stderr_redirect {
            let file = self.open_file(path, cmd.stderr_redirect_append)?;
            Stdio::from(file)
        } else {
            Stdio::inherit()
        };

        let mut child = ProcessCommand::new(&cmd.command) // Usamos nome simples conforme pedido
            .args(&cmd.args)
            .stdin(stdin)
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
            .map_err(|e| format!("Failed to start {}: {}", cmd.command, e))?;

        
        if let PipeState::Buffer(data) = input {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(data); 
            }
        }

        if creates_pipe {
            Ok((PipeState::Process(child), ShellStatus::Continue))
        } else {
            child.wait().map_err(|e| e.to_string())?;
            Ok((PipeState::None, ShellStatus::Continue))
        }
    }

    fn open_file(&self, path: &str, append: bool) -> Result<File, String> {
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(!append)
            .append(append)
            .open(path)
            .map_err(|e| format!("Failed to open {}: {}", path, e))
    }
}