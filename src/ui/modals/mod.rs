pub mod adding_modal;
pub mod confirming_modal;
pub mod creating_group_modal;
pub mod editing_modal;

use color_eyre::eyre;
use crossterm::event::KeyEvent;
use ratatui::{Frame, widgets::ListState};

use crate::{todo_manager::TodoManager, ui::Action};

pub trait Modal {
    fn render(&self, frame: &mut Frame);
    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        list_state: &mut ListState,
        todo_manager: &mut TodoManager,
    ) -> eyre::Result<Action>;
}
