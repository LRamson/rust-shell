pub fn parse_input(input: &str) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    let mut current_arg: String = String::new();
    let mut in_quotes:bool = false;

    for c in input.chars() {
        match c {
            '\'' => {
                in_quotes = !in_quotes;
            },
            c if c.is_whitespace()=> {
                if in_quotes {
                    current_arg.push(c);
                } else if !current_arg.is_empty() {
                    args.push(current_arg);
                    current_arg = String::new();
                }
            },
            _ => {
                current_arg.push(c);
            }
        }
    }

    args
}