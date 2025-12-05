const SPECIAL_CHARS: &[&'static str] = &["\"", "\\"];

#[derive(Debug)]
pub struct ParsedCommand {
    pub command: String,
    pub args: Vec<String>,
    pub stdout_redirect: Option<String>,
    pub stderr_redirect: Option<String>,
}


pub fn parse_command_line(input: &str) -> Option<ParsedCommand> {
    let tokens = tokenize_input(input); 
    
    if tokens.is_empty() {
        return None;
    }

    let command = tokens[0].clone();
    let mut args = Vec::new();
    let mut stdout_redirect = None;
    let mut stderr_redirect = None;

    let mut iter = tokens.iter().skip(1).peekable();

    while let Some(token) = iter.next() {
        if token == ">" || token == "1>" {
            if let Some(path) = iter.next() {
                stdout_redirect = Some(path.clone());
            } else {
                eprintln!("Syntax error: expected file path after redirect");
            }
        } else if token == "2>" {
            if let Some(_path) = iter.next() {
                stderr_redirect = Some(_path.clone());
            } else {
                eprintln!("Syntax error: expected file path after redirect");
            }

        } else {
            args.push(token.clone());
        }
    }

    Some(ParsedCommand {
        command,
        args,
        stdout_redirect,
        stderr_redirect,
    })
}


pub fn tokenize_input(input: &str) -> Vec<String> {
    let mut chars = input.chars().peekable();

    let mut args: Vec<String> = Vec::new();
    let mut current_arg: String = String::new();

    let mut in_quotes:bool = false;
    let mut in_double_quotes: bool = false;
    let mut escape_next: bool = false;

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if (!in_double_quotes && !in_quotes && !escape_next) || 
                (chars.peek().map_or(false, |next_c| SPECIAL_CHARS.contains(&next_c.to_string().as_str())) && !escape_next && in_double_quotes){
                    escape_next = true;
                } else {
                    current_arg.push(c);
                    escape_next = false;
                }
            }
            '\'' => {
                if !in_double_quotes && !escape_next {
                    in_quotes = !in_quotes;
                } else {
                    current_arg.push(c);
                    escape_next = false;
                }
            },
            '"' => {
                if !in_quotes && !escape_next {
                    in_double_quotes = !in_double_quotes;
                } else {
                    current_arg.push(c);
                    escape_next = false;
                }
            },
            c if c.is_whitespace()=> {
                if in_quotes || in_double_quotes || escape_next {
                    current_arg.push(c);
                    escape_next = false;
                } else if !current_arg.is_empty() {
                    args.push(current_arg);
                    current_arg = String::new();
                }
            },
            _ => {
                current_arg.push(c);
                escape_next = false;
            }
        }
    }
    args
}