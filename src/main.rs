mod app;
mod storage;
mod ui;
use crate::{
    app::{App, CurrentAction, CurrentScreen},
    ui::{adding_todo_modal, confirm_action, editing_todo_modal, main_screen},
};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
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
                handle_key_event(key_event, app)?;
            }
            _ => {}
        }
    }

    Ok(())
}

fn handle_key_event(key_event: KeyEvent, app: &mut App) -> color_eyre::Result<()> {
    match app.current_screen {
        CurrentScreen::Main => match key_event.code {
            KeyCode::Down => app.select_next_item(),
            KeyCode::Up => app.select_previous_item(),
            KeyCode::Char('a') => app.current_screen = CurrentScreen::Adding,
            KeyCode::Char('e') => {
                if let Some(selected_todo_index) = app.state.selected()
                    && let Some(selected_todo) = app.todos.get(selected_todo_index)
                {
                    app.edit_todo_input.clear();
                    app.edit_todo_input.push_str(&selected_todo.content);
                    app.current_screen = CurrentScreen::Editing;
                }
            }
            KeyCode::Char('t') => app.toggle_done()?,
            KeyCode::Char('d') => {
                if !app.todos.is_empty() {
                    app.current_action = CurrentAction::Delete;
                    app.current_screen = CurrentScreen::ConfirmAction;
                }
            }
            KeyCode::Char('q') => app.quit(),

            _ => {}
        },

        CurrentScreen::Adding => match key_event.code {
            KeyCode::Enter => {
                if !app.add_todo_input.is_empty() {
                    app.add_todo()?;
                }

                app.current_screen = CurrentScreen::Main;
            }

            KeyCode::Esc => {
                app.add_todo_input.clear();

                app.current_screen = CurrentScreen::Main;
            }

            KeyCode::Backspace => {
                _ = app.add_todo_input.pop();
            }

            _ => {
                if let Some(char) = key_event.code.as_char() {
                    app.add_todo_input.push(char);
                }
            }
        },

        CurrentScreen::Editing => match key_event.code {
            KeyCode::Enter => {
                if !app.edit_todo_input.is_empty() {
                    app.current_action = CurrentAction::Edit;
                    app.current_screen = CurrentScreen::ConfirmAction;
                }
            }
            KeyCode::Esc => {
                app.current_screen = CurrentScreen::Main;
            }

            KeyCode::Backspace => {
                _ = app.edit_todo_input.pop();
            }

            _ => {
                if let Some(char) = key_event.code.as_char() {
                    app.edit_todo_input.push(char);
                }
            }
        },

        CurrentScreen::ConfirmAction => match key_event.code {
            KeyCode::Char('y') => {
                match app.current_action {
                    CurrentAction::Delete => {
                        app.remove_todo()?;
                    }

                    CurrentAction::Edit => {
                        app.edit_todo()?;
                    }

                    CurrentAction::None => {}
                }

                app.current_action = CurrentAction::None;
                app.current_screen = CurrentScreen::Main;
            }

            KeyCode::Char('n') => {
                app.current_action = CurrentAction::None;
                app.current_screen = CurrentScreen::Main;
            }

            _ => {}
        },
    }

    Ok(())
}

fn render(frame: &mut Frame, app: &mut App) {
    main_screen(frame, app);

    match app.current_screen {
        CurrentScreen::Main => {}
        CurrentScreen::Adding => {
            adding_todo_modal(frame, app);
        }
        CurrentScreen::Editing => {
            editing_todo_modal(frame, app);
        }
        CurrentScreen::ConfirmAction => {
            confirm_action(frame);
        }
    }
}
