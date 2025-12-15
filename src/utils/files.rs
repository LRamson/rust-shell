use std::fs::{File, OpenOptions};

pub fn open_file(path: &str, append: bool) -> Result<File, String> {
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(!append)
            .append(append)
            .open(path)
            .map_err(|e| format!("Failed to open {}: {}", path, e))
}