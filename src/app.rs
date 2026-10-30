use std::time::SystemTime;

use ratatui::widgets::ListState;

pub struct Todo {
    pub content: String,
    pub is_done: bool,
    pub id: usize,
    pub time: SystemTime,
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
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
            current_screen: CurrentScreen::Main,
            state: ListState::default().with_selected(Some(0)),
            should_quit: false,
        }
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
