use color_eyre::eyre::{self, Ok};
use crossterm::event::KeyCode;
use ratatui::{
    layout::{Alignment, Constraint},
    widgets::{Block, Paragraph},
};

use crate::{
    app::App,
    ui::modals::{
        Modal, ModalAction,
        confirming_modal::{ConfirmActions, ConfirmingModal},
    },
};

pub struct EditingModal {
    pub input: String,
}

impl Modal for EditingModal {
    fn render(&self, frame: &mut ratatui::prelude::Frame) {
        let width = usize::from(((60 * (frame.area().width) / 100).saturating_sub(1)).max(1));
        let number_of_line = (self.input.len()).div_ceil(width).max(1);
        let area = frame.area().centered(
            Constraint::Percentage(60),
            Constraint::Length(u16::try_from(number_of_line + 2).unwrap_or(u16::MAX)),
        );

        let popup = Paragraph::new(self.input.as_str()).block(
            Block::bordered()
                .title_alignment(Alignment::Center)
                .title_top("Edit TO-DO")
                .title_bottom("[ESC] Close [Enter] Edit"),
        );
        frame.render_widget(popup, area);
    }

    fn handle_key_event(
        &mut self,
        _: &mut App,
        key_event: crossterm::event::KeyEvent,
    ) -> eyre::Result<ModalAction> {
        match key_event.code {
            KeyCode::Enter => {
                if self.input.is_empty() {
                    Ok(ModalAction::Close)
                } else {
                    Ok(ModalAction::Switch(Box::new(ConfirmingModal {
                        action_to_confirm: ConfirmActions::Edit {
                            input: self.input.clone(),
                        },
                    })))
                }
            }
            KeyCode::Esc => Ok(ModalAction::Close),

            KeyCode::Backspace => {
                _ = self.input.pop();

                Ok(ModalAction::Keep)
            }

            _ => {
                if let Some(char) = key_event.code.as_char() {
                    self.input.push(char);
                }

                Ok(ModalAction::Keep)
            }
        }
    }
}
