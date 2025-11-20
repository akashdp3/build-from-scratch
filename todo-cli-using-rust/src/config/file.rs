use std::{fs, io};

use crate::commands::Todo;

const FILE_PATH: &str = "./todo.json";

pub fn save(todo: &Todo) -> Result<(), std::io::Error> {
    let content_json = serde_json::to_string_pretty(todo)?;
    fs::write(FILE_PATH, content_json)
}

pub fn load() -> io::Result<String> {
    let file_content = fs::read_to_string(FILE_PATH)?;

    Ok(file_content)
}
