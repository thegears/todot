use ratatui::{
    Frame,
    layout::Constraint,
    style::{Color, Style},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};

use crate::app::App;

pub fn main_screen(frame: &mut Frame, app: &mut App) {
    let items: Vec<ListItem> = app
        .todos
        .iter()
        .map(|todo| ListItem::new(todo.content.as_str()))
        .collect();

    let list = List::new(items).highlight_symbol(">");

    frame.render_stateful_widget(list, frame.area(), &mut app.state);
}

pub fn adding_todo_modal(frame: &mut Frame, app: &mut App) {
    let area = frame
        .area()
        .centered(Constraint::Percentage(60), Constraint::Length(10));
    let popup =
        Paragraph::new(app.add_todo_input.as_str()).block(Block::bordered().title("Add TO-DO"));
    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);
}
