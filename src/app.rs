use std::time::SystemTime;

pub struct Todo {
    pub content: String,
    pub is_done: bool,
    pub id: usize,
    pub time: SystemTime,
}

pub struct App {
    pub todos: Vec<Todo>,
}

impl App {
    pub fn new() -> Self {
        Self { todos: Vec::new() }
    }

    pub fn add_todo(&mut self, content: String) {}

    pub fn remove_todo(&mut self, id: usize) {}

    pub fn edit_todo(&mut self, id: usize, new_content: String) {}

    pub fn toggle_done(&mut self, id: usize) {}
}
