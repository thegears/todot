use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Todo {
    pub content: String,
    pub is_done: bool,
    pub id: usize,
}

impl Todo {
    pub fn update_content(&mut self, new_content: &str) {
        self.content.clear();
        self.content.push_str(new_content);
    }

    pub fn toggle_done(&mut self) {
        self.is_done = !self.is_done;
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Group {
    pub id: usize,
    pub name: String,
    pub is_collapsed: bool,
}

impl Group {
    pub fn toggle_collapse(&mut self) {
        self.is_collapsed = !self.is_collapsed;
    }
}
