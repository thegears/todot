use std::{env, fs, path::PathBuf};

use color_eyre::eyre::{self, OptionExt};
use serde::Deserialize;

use crate::{models::Todo, storage::Storage};

#[derive(Debug, PartialEq, Deserialize)]
pub struct JsonFileData {
    todos: Vec<Todo>,
}

pub struct JsonStorage;

impl JsonStorage {
    pub fn new() -> Self {
        Self
    }
}

fn get_path() -> color_eyre::Result<PathBuf> {
    let is_global = env::args().any(|arg| arg == "--global" || arg == "-g");

    let json_path: PathBuf;

    if is_global {
        let local_dir = dirs::data_local_dir().ok_or_eyre("Error on finding local dir!")?;
        let todot_dir_path = local_dir.join("todot");
        json_path = todot_dir_path.join("todot.json");

        if !todot_dir_path.exists() {
            fs::create_dir_all(todot_dir_path)?;
        }
    } else {
        json_path = env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("todot.json");
    }

    if !json_path.exists() {
        fs::write(&json_path, r#"{"todos":[]}"#)?;
    }

    Ok(json_path)
}

impl Storage for JsonStorage {
    fn load(&self) -> eyre::Result<Vec<Todo>> {
        let json_path = get_path()?;
        let json_string = fs::read_to_string(json_path)?;
        let json: JsonFileData = serde_json::from_str(json_string.as_str())?;
        let todos = json.todos;

        Ok(todos)
    }

    fn save(&self, todos: &[Todo]) -> eyre::Result<()> {
        let json_path = get_path()?;

        let todos_string = serde_json::to_string_pretty(todos)?;

        fs::write(json_path, format!(r#"{{"todos": {todos_string}}}"#))?;
        Ok(())
    }
}
