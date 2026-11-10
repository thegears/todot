pub mod adding_modal;
pub mod confirming_modal;
pub mod creating_group_modal;
pub mod editing_modal;

use crate::ui::modals::{
    adding_modal::AddingModal, confirming_modal::ConfirmingModal,
    creating_group_modal::CreatingGroupModal, editing_modal::EditingModal,
};

use color_eyre::eyre;
use crossterm::event::KeyEvent;
use enum_dispatch::enum_dispatch;
use ratatui::{Frame, widgets::ListState};

use crate::{todo_manager::TodoManager, ui::Action};

#[enum_dispatch]
pub trait Modal {
    fn render(&self, frame: &mut Frame);
    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        list_state: &mut ListState,
        todo_manager: &mut TodoManager,
    ) -> eyre::Result<Action>;
}

#[enum_dispatch(Modal)]
pub enum CurrentModal {
    Adding(AddingModal),
    Confirming(ConfirmingModal),
    CreatingGroup(CreatingGroupModal),
    Editing(EditingModal),
}
