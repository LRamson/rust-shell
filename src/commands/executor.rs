use std::process::{Command as ProcessCommand, Stdio, Child};
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use super::{CommandRegistry, ShellStatus, Command};
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

            // 1. Configuração de I/O
            let (stdin, stdout, stderr, is_pipe_output) = self.configure_io(
                cmd, 
                &mut previous_process, 
                is_last
            )?;

            // 2. Dispatch e Captura de Status
            // Agora recebemos (Processo?, Status)
            let (child_process, status) = self.dispatch_command(cmd, stdin, stdout, stderr)?;

            // CORREÇÃO 1: Se o comando pediu para sair (ex: "exit"), paramos tudo e retornamos.
            if let ShellStatus::Exit = status {
                return Ok(ShellStatus::Exit);
            }

            // 3. Gestão de Pipeline
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

    // --- Configuração de I/O ---

    fn configure_io(
        &self, 
        cmd: &ParsedCommand, 
        previous_process: &mut Option<Child>, 
        is_last: bool
    ) -> Result<(Stdio, Stdio, Stdio, bool), String> {
        
        // Stdin
        let stdin = if let Some(mut child) = previous_process.take() {
            if let Some(out) = child.stdout.take() {
                Stdio::from(out)
            } else {
                Stdio::null() 
            }
        } else {
            Stdio::inherit()
        };

        // Stdout
        let (stdout, is_pipe) = if let Some(path) = &cmd.stdout_redirect {
            let file = self.open_file(path, cmd.stdout_redirect_append)?;
            (Stdio::from(file), false)
        } else if !is_last {
            (Stdio::piped(), true)
        } else {
            (Stdio::inherit(), false)
        };

        // Stderr
        let stderr = if let Some(path) = &cmd.stderr_redirect {
            let file = self.open_file(path, cmd.stderr_redirect_append)?;
            Stdio::from(file)
        } else {
            Stdio::inherit()
        };

        Ok((stdin, stdout, stderr, is_pipe))
    }

    // --- Dispatcher ---

    // Retorna: (Option<Child>, ShellStatus)
    fn dispatch_command(
        &self, 
        cmd: &ParsedCommand, 
        stdin: Stdio, 
        stdout: Stdio, 
        stderr: Stdio
    ) -> Result<(Option<Child>, ShellStatus), String> {
        
        // 1. Tenta Builtin
        if let Some(builtin) = self.registry.get_builtin(&cmd.command) {
            // CORREÇÃO 2: Builtins precisam de Writers, não Stdio.
            // Ignoramos o 'stdout' (Stdio) passado e criamos um Writer adequado aqui.
            // Nota: Isso não suporta pipe 'echo | wc' ainda, mas suporta redirects 'echo > file'.
            let mut writer = self.get_builtin_writer(cmd)?;
            
            // Executamos e capturamos o status (ex: Exit)
            let status = builtin.execute(&cmd.args, self.registry, &mut *writer)?;
            return Ok((None, status));
        }
        
        // 2. Tenta Executável
        let exists = self.registry.get_executable_path(&cmd.command)
            .or_else(|| self.registry.executables.get(&cmd.command).cloned())
            .is_some();

        if exists {
            let child = self.run_external(&cmd.command, &cmd.args, stdin, stdout, stderr)?;
            return Ok((Some(child), ShellStatus::Continue));
        }

        Err(format!("{}: command not found", cmd.command))
    }

    // --- Helpers de Execução ---

    fn run_external(&self, program_name: &str, args: &[String], stdin: Stdio, stdout: Stdio, stderr: Stdio) -> Result<Child, String> {
        ProcessCommand::new(program_name)
            .args(args)
            .stdin(stdin)
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
            .map_err(|e| format!("Failed to start {}: {}", program_name, e))
    }

    // CORREÇÃO 2 (Continuação): Helper para abrir ficheiros para builtins
    fn get_builtin_writer(&self, cmd: &ParsedCommand) -> Result<Box<dyn Write>, String> {
        if let Some(path) = &cmd.stdout_redirect {
            let file = self.open_file(path, cmd.stdout_redirect_append)?;
            Ok(Box::new(file))
        } else {
            // Se não tem redirect, vai para o ecrã.
            // (No futuro, aqui verificaríamos se devia ir para um Pipe)
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