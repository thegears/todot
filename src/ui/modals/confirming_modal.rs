use color_eyre::eyre;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint},
    widgets::Block,
};

use crate::{
    app::App,
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

    fn handle_key_event(&mut self, app: &mut App, key_event: KeyEvent) -> eyre::Result<Action> {
        if key_event.code == KeyCode::Char('y') {
            match &mut self.action_to_confirm {
                ConfirmActions::Delete => {
                    app.todo_manager.delete_todo(&app.state)?;
                }

                ConfirmActions::Edit { input } => {
                    app.todo_manager.edit_todo(input.as_str(), &app.state)?;
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
