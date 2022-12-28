use std::{path::PathBuf};

use dirs::home_dir;

use crate::result::ResultWrap;


pub fn get_file_path() -> ResultWrap<PathBuf> {
    let home_dir = home_dir().ok_or("Failed to resolve home directory")?;
    let file_path = std::path::Path::new(&home_dir).join(".tmonitor/time.txt");
    Ok(file_path)
}

pub fn extract_timstamps_from_line(line: &str) -> (&str, &str) {
    let timestamps_last_line: Vec<&str> = line.split_whitespace().collect();
    let first = timestamps_last_line[0];
    let second = timestamps_last_line[1];
    return (first, second);
}