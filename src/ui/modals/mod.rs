pub mod adding_modal;
pub mod confirming_modal;
pub mod editing_modal;

use color_eyre::eyre;
use crossterm::event::KeyEvent;
use ratatui::Frame;

use crate::{app::App, ui::Action};

pub trait Modal {
    fn render(&self, frame: &mut Frame);
    fn handle_key_event(&mut self, app: &mut App, key_event: KeyEvent) -> eyre::Result<Action>;
}
