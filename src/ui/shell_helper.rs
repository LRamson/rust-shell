use rustyline::completion::Completer;
use rustyline::hint::Hinter;
use rustyline::highlight::Highlighter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper, Result};

pub struct ShellHelper {
    pub commands: Vec<String>,
}

impl ShellHelper {
    pub fn new(commands: Vec<String>) -> Self {
        Self { commands }
    }
}

impl Completer for ShellHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        _pos: usize,
        _ctx: &Context,
    ) -> Result<(usize, Vec<String>)> {
        let mut candidates = Vec::new();

        if line.is_empty() {
             return Ok((0, candidates));
        }

        for command in &self.commands {
            if command.starts_with(line) {
                candidates.push(format!("{}", command));
            }
        }

        Ok((0, candidates))
    }
}

impl Helper for ShellHelper {}

impl Hinter for ShellHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context) -> Option<String> {
        None
    }
}

impl Highlighter for ShellHelper {}

impl Validator for ShellHelper {
    fn validate(&self, _ctx: &mut rustyline::validate::ValidationContext) -> Result<rustyline::validate::ValidationResult> {
        Ok(rustyline::validate::ValidationResult::Valid(None))
    }
}