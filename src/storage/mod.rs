mod json;

use color_eyre::eyre;

use crate::models::Todo;

pub trait Storage {
    fn load(&self) -> eyre::Result<Vec<Todo>>;
    fn save(&self, todos: &[Todo]) -> eyre::Result<()>;
}

pub use json::JsonStorage;
