mod parser;
mod files;

pub use parser::ParsedCommand;
pub use parser::parse_input;
pub use files::open_file;