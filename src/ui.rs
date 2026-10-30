use ratatui::{Frame, widgets::{List, ListItem}};

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
