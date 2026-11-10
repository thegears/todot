use crate::storage::JsonStorage;

use crate::todo_manager::TodoManager;
use crate::ui::modals::CurrentModal;

use ratatui::widgets::ListState;

pub struct App {
    pub todo_manager: TodoManager,
    pub state: ListState,
    pub current_modal: Option<CurrentModal>,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> color_eyre::Result<Self> {
        let storage = Box::new(JsonStorage::new());
        let todo_manager = TodoManager::new(storage)?;

        Ok(Self {
            todo_manager,
            state: ListState::default().with_selected(Some(0)),
            current_modal: None,
            should_quit: false,
        })
    }
}
