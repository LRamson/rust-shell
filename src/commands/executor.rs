use std::process::{Command as ProcessCommand, Stdio, Child};
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use super::{CommandRegistry, ShellStatus};
use crate::utils::ParsedCommand;

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

        let mut previous_process: Option<Child> = None;
        let mut iter = pipeline.iter().peekable();

        while let Some(cmd) = iter.next() {
            let is_last = iter.peek().is_none();

            let (stdin, stdout, stderr, is_pipe_output) = self.configure_io(
                cmd, 
                &mut previous_process, 
                is_last
            )?;

            let (child_process, status) = self.dispatch_command(cmd, stdin, stdout, stderr)?;

            if let ShellStatus::Exit = status {
                return Ok(ShellStatus::Exit);
            }

            if is_pipe_output {
                previous_process = child_process;
            } else {
                if let Some(mut child) = child_process {
                    child.wait().map_err(|e| e.to_string())?;
                }
                previous_process = None;
            }
        }

        Ok(ShellStatus::Continue)
    }


    fn configure_io(
        &self, 
        cmd: &ParsedCommand, 
        previous_process: &mut Option<Child>, 
        is_last: bool
    ) -> Result<(Stdio, Stdio, Stdio, bool), String> {
        
        let stdin = if let Some(mut child) = previous_process.take() {
            if let Some(out) = child.stdout.take() { Stdio::from(out) } else { Stdio::null() }
        } else {
            Stdio::inherit()
        };

        let (stdout, is_pipe) = if let Some(path) = &cmd.stdout_redirect {
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

        Ok((stdin, stdout, stderr, is_pipe))
    }


    fn dispatch_command(
        &self, 
        cmd: &ParsedCommand, 
        stdin: Stdio, 
        stdout: Stdio, 
        stderr: Stdio
    ) -> Result<(Option<Child>, ShellStatus), String> {
        
        if let Some(builtin) = self.registry.get_builtin(&cmd.command) {
            let mut stdout_writer = self.get_builtin_writer(cmd)?;
            
            let result = builtin.execute(&cmd.args, self.registry, &mut *stdout_writer);

            match result {
                Ok(status) => {
                    return Ok((None, status));
                }
                Err(error_msg) => {
                    if let Some(path) = &cmd.stderr_redirect {
                        let mut file = self.open_file(path, cmd.stderr_redirect_append)?;
                        writeln!(file, "{}", error_msg).map_err(|e| e.to_string())?;
                        
                        return Ok((None, ShellStatus::Continue));
                    } else {
                        return Err(error_msg);
                    }
                }
            }
        }
        

        if self.registry.get_executable(&cmd.command).is_some() {
            let child = self.run_external(&cmd.command, &cmd.args, stdin, stdout, stderr)?;
            return Ok((Some(child), ShellStatus::Continue));
        }

        Err(format!("{}: command not found", cmd.command))
    }


    fn run_external(&self, program_name: &str, args: &[String], stdin: Stdio, stdout: Stdio, stderr: Stdio) -> Result<Child, String> {
        ProcessCommand::new(program_name)
            .args(args)
            .stdin(stdin)
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
            .map_err(|e| format!("Failed to start {}: {}", program_name, e))
    }

    fn get_builtin_writer(&self, cmd: &ParsedCommand) -> Result<Box<dyn Write>, String> {
        if let Some(path) = &cmd.stdout_redirect {
            let file = self.open_file(path, cmd.stdout_redirect_append)?;
            Ok(Box::new(file))
        } else {
            Ok(Box::new(io::stdout()))
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