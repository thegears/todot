use color_eyre::eyre::{self, Ok};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint},
    widgets::{Block, Paragraph, Wrap},
};

use crate::{
    app::App,
    ui::modals::{Modal, ModalAction},
};

pub struct AddingModal {
    pub input: String,
}

impl Modal for AddingModal {
    fn render(&self, frame: &mut Frame) {
        let width = usize::from(((60 * (frame.area().width) / 100).saturating_sub(1)).max(1));
        let number_of_line = (self.input.len()).div_ceil(width).max(1);
        let area = frame.area().centered(
            Constraint::Percentage(60),
            Constraint::Length(u16::try_from(number_of_line + 2).unwrap_or(u16::MAX)),
        );

        let popup = Paragraph::new(self.input.as_str())
            .wrap(Wrap { trim: true })
            .block(
                Block::bordered()
                    .title_alignment(Alignment::Center)
                    .title_top("Add TO-DO")
                    .title_bottom("[ESC] Close [Enter] Add"),
            );
        frame.render_widget(popup, area);
    }

    fn handle_key_event(
        &mut self,
        app: &mut App,
        key_event: KeyEvent,
    ) -> eyre::Result<ModalAction> {
        match key_event.code {
            KeyCode::Enter => {
                if !&self.input.is_empty() {
                    app.todo_manager.create_todo(&self.input)?;
                    app.state.select_last();
                    self.input.clear();
                }

                Ok(ModalAction::Close)
            }

            KeyCode::Esc => {
                self.input.clear();

                Ok(ModalAction::Close)
            }

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
