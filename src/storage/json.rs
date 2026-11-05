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

#[derive(Debug, PartialEq, Clone, Copy)]
enum FileType {
    Real,
    Tmp,
    Bak,
}

fn get_path(file_type: FileType) -> color_eyre::Result<PathBuf> {
    let is_global = env::args().any(|arg| arg == "--global" || arg == "-g");

    let json_pathbuf: PathBuf;

    let json_path: String = match file_type {
        FileType::Real => String::from("todot.json"),
        FileType::Bak => String::from(".todot.json.bak"),
        FileType::Tmp => String::from(".todot.json.tmp"),
    };

    if is_global {
        let local_dir = dirs::data_local_dir().ok_or_eyre("Error on finding local dir!")?;
        let todot_dir_path = local_dir.join("todot");
        json_pathbuf = todot_dir_path.join(json_path);

        if !todot_dir_path.exists() {
            fs::create_dir_all(todot_dir_path)?;
        }
    } else {
        json_pathbuf = env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(json_path);
    }

    if !json_pathbuf.exists() {
        fs::write(&json_pathbuf, r#"{"todos":[]}"#)?;
    }

    Ok(json_pathbuf)
}

fn load() -> eyre::Result<JsonFileData> {
    let json_pathbuf = get_path(FileType::Real)?;
    let json_string = fs::read_to_string(json_pathbuf)?;
    let json: JsonFileData = serde_json::from_str(json_string.as_str())?;

    Ok(json)
}

impl Storage for JsonStorage {
    fn get_todos(&self) -> eyre::Result<Vec<Todo>> {
        let json: JsonFileData = load()?;

        Ok(json.todos)
    }

    fn save(&self, todos: &[Todo]) -> eyre::Result<()> {
        let real_json_pathbuf = get_path(FileType::Real)?;
        let tmp_json_pathbuf = get_path(FileType::Tmp)?;
        let bak_json_pathbuf = get_path(FileType::Bak)?;

        let todos_string = serde_json::to_string_pretty(todos)?;

        fs::write(&tmp_json_pathbuf, format!(r#"{{"todos": {todos_string}}}"#))?;

        fs::rename(&real_json_pathbuf, bak_json_pathbuf)?;
        fs::rename(tmp_json_pathbuf, real_json_pathbuf)?;
        Ok(())
    }
}
