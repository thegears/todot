mod app;
mod models;
mod storage;
mod todo_manager;
mod ui;

use crate::ui::modals::Modal;

use crate::{
    app::App,
    ui::{Action, main_screen::MainScreen},
};

use crossterm::event::{Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut app = App::new()?;
    ratatui::run(|terminal| tui(terminal, &mut app))?;
    Ok(())
}

fn tui(terminal: &mut DefaultTerminal, app: &mut App) -> color_eyre::Result<()> {
    while !app.should_quit {
        terminal.draw(|frame| render(frame, app))?;
        match crossterm::event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                if let Some(modal) = &mut app.current_modal {
                    let action =
                        modal.handle_key_event(key_event, &mut app.state, &mut app.todo_manager)?;

                    match action {
                        Action::Close => app.current_modal = None,
                        Action::None => {}
                        Action::Open(next_modal) => app.current_modal = Some(next_modal),
                    }
                } else {
                    let action = MainScreen::handle_key_event(
                        key_event,
                        &mut app.state,
                        &mut app.todo_manager,
                        &mut app.should_quit,
                    )?;

                    if let Action::Open(next_modal) = action {
                        app.current_modal = Some(next_modal);
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn render(frame: &mut Frame, app: &mut App) {
    MainScreen::render(frame, &mut app.todo_manager, &mut app.state);

    if let Some(modal) = &mut app.current_modal {
        modal.render(frame);
    }
}
