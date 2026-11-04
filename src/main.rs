mod app;
mod models;
mod storage;
mod todo_manager;
mod ui;

use crate::{
    app::App,
    ui::{
        main_screen,
        modals::{
            ModalAction,
            adding_modal::AddingModal,
            confirming_modal::{ConfirmActions, ConfirmingModal},
            editing_modal::EditingModal,
        },
    },
};

use color_eyre::eyre::Ok;
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
                if app.current_modal.is_some() {
                    if let Some(mut modal) = app.current_modal.take() {
                        let result = modal.handle_key_event(app, key_event)?;

                        match result {
                            ModalAction::Close => app.current_modal = None,
                            ModalAction::Keep => app.current_modal = Some(modal),
                            ModalAction::Switch(next_modal) => app.current_modal = Some(next_modal),
                        }
                    }
                } else {
                    let key_event = lowercase_key_event(key_event);
                    match key_event.code {
                        KeyCode::Down => app.state.select_next(),
                        KeyCode::Up => app.state.select_previous(),
                        KeyCode::Char('a') => {
                            app.current_modal = Option::Some(Box::new(AddingModal {
                                input: String::new(),
                            }));
                        }
                        KeyCode::Char('e') => {
                            if let Some(selected_todo_index) = app.state.selected()
                                && let Some(selected_todo) =
                                    app.todo_manager.todos.get(selected_todo_index)
                            {
                                app.current_modal = Option::Some(Box::new(EditingModal {
                                    input: String::from(&selected_todo.content),
                                }));
                            }
                        }
                        KeyCode::Char('t') => app.todo_manager.toggle_done_todo(&app.state)?,
                        KeyCode::Char('d') => {
                            if !app.todo_manager.todos.is_empty() {
                                app.current_modal = Option::Some(Box::new(ConfirmingModal {
                                    action_to_confirm: ConfirmActions::Delete,
                                }));
                            }
                        }
                        KeyCode::Char('q') => app.quit(),

                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn lowercase_key_event(mut key_event: KeyEvent) -> KeyEvent {
    if let KeyCode::Char(c) = key_event.code {
        key_event.code = KeyCode::Char(c.to_lowercase().next().unwrap_or(c));
    }
    key_event
}

fn render(frame: &mut Frame, app: &mut App) {
    main_screen(frame, &app.todo_manager.todos, &mut app.state);

    if let Some(modal) = &mut app.current_modal {
        modal.render(frame);
    }
}
