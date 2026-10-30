use color_eyre::eyre::OptionExt;
use dirs;
use std::{fs, path::PathBuf};

use serde_derive::Deserialize;

use crate::app::Todo;

#[derive(Debug, PartialEq, Deserialize)]
pub struct AppData {
    todos: Vec<Todo>,
}

fn get_path() -> color_eyre::Result<PathBuf> {
    let local_dir = dirs::data_local_dir().ok_or_eyre("Error on finding local dir!")?;
    let todot_dir_path = local_dir.join("todot");
    let json_path = todot_dir_path.join("todot.json");

    if !todot_dir_path.exists() {
        fs::create_dir_all(todot_dir_path)?;
    }

    if !json_path.exists() {
        fs::write(&json_path, r#"{"todos":[]}"#)?;
    }

    Ok(json_path)
}

pub fn get_todos_from_file() -> color_eyre::Result<Vec<Todo>> {
    let json_path = get_path()?;
    let json_string = fs::read_to_string(json_path)?;
    let json: AppData = serde_json::from_str(json_string.as_str())?;
    let todos = json.todos;

    Ok(todos)
}

pub fn update_file(todos: &Vec<Todo>) -> color_eyre::Result<()> {
    let json_path = get_path()?;

    let todos_string = serde_json::to_string_pretty(todos)?;

    fs::write(json_path, format!(r#"{{"todos": {}}}"#, todos_string))?;
    Ok(())
}
