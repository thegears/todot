use ratatui::{
    Frame,
    style::{Color, Style, Styled, Stylize},
    text::{Line, Text},
    widgets::{Block, List, ListItem, ListState, Padding},
};
use textwrap::wrap;

use crate::models::Todo;

pub mod modals;

pub fn main_screen(frame: &mut Frame, todos: &[Todo], list_state: &mut ListState) {
    let outer_block = Block::bordered()
        .title_top(Line::from("Todot").bold().centered())
        .title_bottom(Line::from("[a] Add [e] Edit [d] Delete [t] Toggle Done [q] Quit").centered())
        .padding(Padding::symmetric(1, 1));
    let inner_area = outer_block.inner(frame.area());

    let items: Vec<ListItem> = todos
        .iter()
        .map(|todo| {
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

            ListItem::new(Text::from(lines)).set_style(Style::new().fg(text_color))
        })
        .collect();

    let list = List::new(items)
        .highlight_symbol(">")
        .highlight_style(Style::new().bold());

    frame.render_widget(outer_block, frame.area());

    frame.render_stateful_widget(list, inner_area, list_state);
}
