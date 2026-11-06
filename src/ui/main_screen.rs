use color_eyre::eyre;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    style::{Color, Style, Styled, Stylize},
    text::{Line, Text},
    widgets::{Block, List, ListItem, ListState, Padding},
};
use textwrap::wrap;

use crate::{
    models::TodoGroup,
    todo_manager::{ItemType, TodoManager},
    ui::{
        Action,
        modals::{
            adding_modal::AddingModal,
            confirming_modal::{ConfirmActions, ConfirmingModal},
            creating_group_modal::CreatingGroupModal,
            editing_modal::EditingModal,
        },
    },
};

pub struct MainScreen;

fn lowercase_key_event(mut key_event: KeyEvent) -> KeyEvent {
    if let KeyCode::Char(c) = key_event.code {
        key_event.code = KeyCode::Char(c.to_lowercase().next().unwrap_or(c));
    }
    key_event
}

impl MainScreen {
    pub fn handle_key_event(
        key_event: KeyEvent,
        list_state: &mut ListState,
        todo_manager: &mut TodoManager,
        should_quit: &mut bool,
    ) -> eyre::Result<Action> {
        let key_event = lowercase_key_event(key_event);
        let action: Action = match key_event.code {
            KeyCode::Down => {
                list_state.select_next();
                Action::None
            }
            KeyCode::Up => {
                list_state.select_previous();
                Action::None
            }
            KeyCode::Char('a') => Action::Open(Box::new(AddingModal {
                input: String::new(),
            })),
            KeyCode::Char('g') => Action::Open(Box::new(CreatingGroupModal {
                input: String::new(),
            })),
            KeyCode::Enter => {
                if let Some(selected_thing_index) = list_state.selected() {
                    let items = todo_manager.get_all_items();

                    if let Some(selected_thing) = items.get(selected_thing_index)
                        && let ItemType::Group(group_id) = selected_thing
                        && let Some(group) = todo_manager.groups.get_mut(*group_id)
                    {
                        group.toggle_collapse();
                    }
                }
                Action::None
            }
            KeyCode::Char('e') => {
                if let Some(selected_thing_index) = list_state.selected() {
                    let items = todo_manager.get_all_items();

                    if let Some(selected_thing) = items.get(selected_thing_index)
                        && let ItemType::Todo(todo_id) = selected_thing
                        && let Some(todo) = todo_manager.todos.iter_mut().find(|t| t.id == *todo_id)
                    {
                        Action::Open(Box::new(EditingModal {
                            input: String::from(&todo.content),
                        }))
                    } else {
                        Action::None
                    }
                } else {
                    Action::None
                }
            }
            KeyCode::Char('t') => {
                todo_manager.toggle_done_todo(list_state)?;
                Action::None
            }
            KeyCode::Char('d') => Action::Open(Box::new(ConfirmingModal {
                action_to_confirm: ConfirmActions::Delete,
            })),
            KeyCode::Char('q') => {
                *should_quit = !(*should_quit);
                Action::None
            }

            _ => Action::None,
        };

        Ok(action)
    }

    pub fn render(frame: &mut Frame, todo_manager: &mut TodoManager, list_state: &mut ListState) {
        let outer_block = Block::bordered()
            .title_top(Line::from("Todot").bold().centered())
            .title_bottom(
                Line::from("[a] Add [e] Edit [d] Delete [t] Toggle Done [q] Quit").centered(),
            )
            .padding(Padding::symmetric(1, 1));
        let inner_area = outer_block.inner(frame.area());

        let todo_manager_items = todo_manager.get_all_items();
        let mut list_items: Vec<ListItem> = Vec::new();

        for item in todo_manager_items {
            match item {
                ItemType::Todo(todo_id) => {
                    if let Some(todo) = todo_manager.todos.iter().find(|t| t.id == todo_id) {
                        let text_prefix: &str;
                        let text_color: Color;

                        if todo.is_done {
                            text_prefix = "[✓]";
                            text_color = Color::DarkGray;
                        } else {
                            text_prefix = "[ ]";
                            text_color = Color::White;
                        }

                        let lines: Vec<Line> = wrap(
                            format!("{} {}", text_prefix, todo.content.as_str()).as_str(),
                            frame.area().width as usize,
                        )
                        .into_iter()
                        .map(|cow| Line::from(cow.into_owned()))
                        .collect();

                        list_items.push(
                            ListItem::new(Text::from(lines)).set_style(Style::new().fg(text_color)),
                        );
                    }
                }

                ItemType::Group(group_id) => {
                    if let Some(group) = todo_manager.groups.get(group_id) {
                        let text_prefix: &str;
                        let text_color: Color;

                        if group.is_collapsed {
                            text_prefix = "▶"; // Kapalıysa sağ ok
                            text_color = Color::Yellow;
                        } else {
                            text_prefix = "▼"; // Açıksa aşağı ok
                            text_color = Color::Yellow;
                        }

                        let lines: Vec<Line> = wrap(
                            format!("{} 📁 {}", text_prefix, group.name.as_str()).as_str(),
                            frame.area().width as usize,
                        )
                        .into_iter()
                        .map(|cow| Line::from(cow.into_owned()))
                        .collect();

                        list_items.push(
                            ListItem::new(Text::from(lines))
                                .set_style(Style::new().fg(text_color).bold()),
                        );
                    }
                }
            }
        }

        let list = List::new(list_items)
            .highlight_symbol(">")
            .highlight_style(Style::new().bold());

        frame.render_widget(outer_block, frame.area());

        frame.render_stateful_widget(list, inner_area, list_state);
    }
}
