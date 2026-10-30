mod app;
mod ui;
use crate::app::{App, CurrentScreen};

use color_eyre::eyre::Ok;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut app = App::new();
    ratatui::run(|terminal| tui(terminal, &mut app))?;
    Ok(())
}

fn tui(terminal: &mut DefaultTerminal, app: &mut App) -> Result<(), color_eyre::eyre::Error> {
    while !app.should_quit {
        terminal.draw(|frame| render(frame, app))?;
        match crossterm::event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                handle_key_event(key_event, app)
            }
            _ => {}
        };
    }

    Ok(())
}

fn handle_key_event(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        KeyCode::Down => app.select_next_item(),
        KeyCode::Up => app.select_previous_item(),
        KeyCode::Char('q') => app.quit(),
        _ => {}
    }
}

fn render(frame: &mut Frame, app: &mut App) {
    ui::main_screen(frame, app);

    match app.current_screen {
        CurrentScreen::Main => {}
        CurrentScreen::Adding => {}
        CurrentScreen::Editing => {}
    }
}
