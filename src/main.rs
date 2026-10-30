mod app;
use crate::app::App;

use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Block, Paragraph},
};

fn main() -> color_eyre::Result<()> {
    let mut app = App::new();
    color_eyre::install()?;
    ratatui::run(|terminal| tui(terminal, &mut app))?;
    Ok(())
}

fn tui(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, app))?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, app: &App) {
    frame.render_widget(Paragraph::new("text").block(Block::new()), frame.area());
}
