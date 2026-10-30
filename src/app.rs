use crate::storage::{self, get_todos};
use color_eyre::eyre::Ok;
use serde_derive::Deserialize;
use std::time::SystemTime;

use ratatui::widgets::ListState;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Todo {
    pub content: String,
    pub is_done: bool,
    pub id: usize,
    pub time: usize,
}

pub enum CurrentScreen {
    Main,
    Adding,
    Editing,
}

pub struct App {
    pub todos: Vec<Todo>,
    pub current_screen: CurrentScreen,
    pub state: ListState,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> color_eyre::Result<Self> {
        let todos = get_todos()?;

        Ok(Self {
            todos,
            current_screen: CurrentScreen::Main,
            state: ListState::default().with_selected(Some(0)),
            should_quit: false,
        })
    }

    pub fn select_next_item(&mut self) {
        self.state.select_next();
    }

    pub fn select_previous_item(&mut self) {
        self.state.select_previous();
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn add_todo(&mut self, content: String) {}

    pub fn remove_todo(&mut self, id: usize) {}

    pub fn edit_todo(&mut self, id: usize, new_content: String) {}

    pub fn toggle_done(&mut self, id: usize) {}
}
