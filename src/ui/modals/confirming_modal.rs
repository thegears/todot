use color_eyre::eyre;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint},
    widgets::{Block, ListState},
};

use crate::{
    todo_manager::TodoManager,
    ui::{Action, modals::Modal},
};

pub enum ConfirmActions {
    Delete,
    Edit { input: String },
    None,
}

pub struct ConfirmingModal {
    pub action_to_confirm: ConfirmActions,
}

impl Modal for ConfirmingModal {
    fn render(&self, frame: &mut Frame) {
        let area = frame
            .area()
            .centered(Constraint::Percentage(60), Constraint::Length(3));

        let popup = Block::bordered()
            .title_alignment(Alignment::Center)
            .title_top("Confirm Action")
            .title_bottom("[y] Yes [n] No");

        frame.render_widget(popup, area);
    }

    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        list_state: &mut ListState,
        todo_manager: &mut TodoManager,
    ) -> eyre::Result<Action> {
        if key_event.code == KeyCode::Char('y') {
            match &mut self.action_to_confirm {
                ConfirmActions::Delete => {
                    todo_manager.delete_thing(list_state)?;
                }

                ConfirmActions::Edit { input } => {
                    todo_manager.edit_todo(input.as_str(), list_state)?;
                }

                ConfirmActions::None => {}
            }
        }

        if matches!(key_event.code, KeyCode::Char('y' | 'n') | KeyCode::Esc) {
            self.action_to_confirm = ConfirmActions::None;

            Ok(Action::Close)
        } else {
            Ok(Action::None)
        }
    }
}

impl ConfirmingModal {
    pub fn new(action_to_confirm: ConfirmActions) -> Self {
        Self { action_to_confirm }
    }
}
