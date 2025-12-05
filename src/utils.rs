const SPECIAL_CHARS: &[&'static str] = &["\"", "\\"];

#[derive(Debug)]
pub struct ParsedCommand {
    pub command: String,
    pub args: Vec<String>,
    pub stdout_redirect: Option<String>,
    pub stderr_redirect: Option<String>,

    pub stdout_redirect_append: bool,
    pub stderr_redirect_append: bool,
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

    let mut stdout_redirect_append = false;
    let mut stderr_redirect_append = false;

    let mut iter = tokens.iter().skip(1).peekable();

    println!("Tokens: {:?}", tokens);
    while let Some(token) = iter.next() {
        match token.as_str() {
            ">>" | "1>>" => {
                println!("Found append stdout redirect");
                if let Some(path) = iter.next() {
                    stdout_redirect = Some(path.clone());
                    stdout_redirect_append = true;
                } else {
                    eprintln!("Syntax error: expected file path after redirect");
                }
            }
            "2>>" => {
                if let Some(path) = iter.next() {
                    stderr_redirect = Some(path.clone());
                    stderr_redirect_append = true;
                } else {
                    eprintln!("Syntax error: expected file path after redirect");
                }
            }
            ">" | "1>" => {
                if let Some(path) = iter.next() {
                    stdout_redirect = Some(path.clone());
                } else {
                    eprintln!("Syntax error: expected file path after redirect");
                }
            }
            "2>" => {
                if let Some(path) = iter.next() {
                    stderr_redirect = Some(path.clone());
                } else {
                    eprintln!("Syntax error: expected file path after redirect");
                }
            }
            _ => {
                args.push(token.clone());
            }
        }
    }

    println!("Parsed: {:?}", ParsedCommand {
        command: command.clone(),
        args: args.clone(),
        stdout_redirect: stdout_redirect.clone(),
        stderr_redirect: stderr_redirect.clone(),
        stdout_redirect_append,
        stderr_redirect_append,
    });

    Some(ParsedCommand {
        command,
        args,
        stdout_redirect,
        stderr_redirect,
        stdout_redirect_append,
        stderr_redirect_append,
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