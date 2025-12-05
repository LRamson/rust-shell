const SPECIAL_CHARS: &[&'static str] = &["\"", "\\"];

pub fn parse_input(input: &str) -> Vec<String> {
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
                (chars.peek().map_or(false, |next_c| SPECIAL_CHARS.contains(&next_c.to_string().as_str())) && !escape_next) {
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