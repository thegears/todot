use crate::storage::JsonStorage;

use crate::todo_manager::TodoManager;

use ratatui::widgets::ListState;

#[derive(PartialEq)]
pub enum CurrentScreen {
    Main,
    Adding,
    Editing,
    ConfirmAction,
}

#[derive(PartialEq)]
pub enum CurrentAction {
    Delete,
    Edit,
    None,
}

pub struct App {
    pub todo_manager: TodoManager,
    pub current_screen: CurrentScreen,
    pub state: ListState,
    pub add_todo_input: String,
    pub edit_todo_input: String,
    pub current_action: CurrentAction,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> color_eyre::Result<Self> {
        let storage = Box::new(JsonStorage::new());
        let todo_manager = TodoManager::new(storage)?;

        Ok(Self {
            todo_manager,
            current_screen: CurrentScreen::Main,
            state: ListState::default().with_selected(Some(0)),
            add_todo_input: String::new(),
            edit_todo_input: String::new(),
            current_action: CurrentAction::None,
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
}
