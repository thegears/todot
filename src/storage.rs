use color_eyre::eyre::OptionExt;
use dirs;
use std::fs;

use serde_derive::Deserialize;

use crate::app::Todo;

#[derive(Debug, PartialEq, Deserialize)]
pub struct AppData {
    todos: Vec<Todo>,
}

fn read_json_file() -> color_eyre::Result<String> {
    let local_dir = dirs::data_local_dir().ok_or_eyre("Error on finding local dir!")?;
    let json_path = local_dir.join("todot").join("todot.json");
    let file = fs::read_to_string(json_path)?;

    Ok(file)
}

pub fn get_todos() -> color_eyre::Result<Vec<Todo>> {
    let json_string = read_json_file()?;
    let json: AppData = serde_json::from_str(json_string.as_str())?;
    let todos = json.todos;

    Ok(todos)
}
