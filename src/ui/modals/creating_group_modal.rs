use color_eyre::eyre::{self, Ok};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint},
    widgets::{Block, ListState, Paragraph, Wrap},
};

use crate::{
    todo_manager::TodoManager,
    ui::{Action, modals::Modal},
};

pub struct CreatingGroupModal {
    pub input: String,
}

impl Modal for CreatingGroupModal {
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
                    .title_top("Create Group")
                    .title_bottom("[ESC] Close [Enter] Create"),
            );
        frame.render_widget(popup, area);
    }

    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        _: &mut ListState,
        todo_manager: &mut TodoManager,
    ) -> eyre::Result<Action> {
        match key_event.code {
            KeyCode::Enter => {
                if !&self.input.is_empty() {
                    todo_manager.create_group(&self.input)?;
                    self.input.clear();
                }

                Ok(Action::Close)
            }

            KeyCode::Esc => {
                self.input.clear();

                Ok(Action::Close)
            }

            KeyCode::Backspace => {
                _ = self.input.pop();

                Ok(Action::None)
            }

            _ => {
                if let Some(char) = key_event.code.as_char() {
                    self.input.push(char);
                }
                Ok(Action::None)
            }
        }
    }
}

impl CreatingGroupModal {
    pub fn new() -> Self {
        Self {
            input: String::new(),
        }
    }
}
