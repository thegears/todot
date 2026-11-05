mod json;

use color_eyre::eyre;

use crate::models::{Group, Todo};

pub trait Storage {
    fn get_todos(&self) -> eyre::Result<Vec<Todo>>;
    fn get_groups(&self) -> eyre::Result<Vec<Group>>;
    fn save(&self, todos: &[Todo], groups: &[Group]) -> eyre::Result<()>;
}

pub use json::JsonStorage;
