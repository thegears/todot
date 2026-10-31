use ratatui::{
    Frame,
    layout::{Alignment, Constraint},
    style::{Color, Style, Styled, Stylize},
    text::Line,
    widgets::{Block, List, ListItem, Padding, Paragraph},
};

use crate::app::App;

pub fn main_screen(frame: &mut Frame, app: &mut App) {
    let outer_block = Block::bordered()
        .title_top(Line::from("Todot").bold().centered())
        .title_bottom(Line::from("[a] Add [e] Edit [d] Delete [t] Toggle Done [q] Quit").centered())
        .padding(Padding::symmetric(1, 1));
    let inner_area = outer_block.inner(frame.area());

    let items: Vec<ListItem> = app
        .todos
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

            ListItem::new(format!("{} {}", text_prefix, todo.content.as_str()))
                .set_style(Style::new().fg(text_color))
        })
        .collect();

    let list = List::new(items)
        .highlight_symbol(">")
        .highlight_style(Style::new().bold());

    frame.render_widget(outer_block, frame.area());

    frame.render_stateful_widget(list, inner_area, &mut app.state);
}

pub fn adding_todo_modal(frame: &mut Frame, app: &mut App) {
    let area = frame
        .area()
        .centered(Constraint::Percentage(60), Constraint::Length(3));
    let popup = Paragraph::new(app.add_todo_input.as_str()).block(
        Block::bordered()
            .title_alignment(Alignment::Center)
            .title_top("Add TO-DO")
            .title_bottom("[ESC] Close [Enter] Add"),
    );
    frame.render_widget(popup, area);
}

pub fn editing_todo_modal(frame: &mut Frame, app: &mut App) {
    let area = frame
        .area()
        .centered(Constraint::Percentage(60), Constraint::Length(3));

    let popup = Paragraph::new(app.edit_todo_input.as_str()).block(
        Block::bordered()
            .title_alignment(Alignment::Center)
            .title_top("Edit TO-DO")
            .title_bottom("[ESC] Close [Enter] Edit"),
    );
    frame.render_widget(popup, area);
}

pub fn confirm_action(frame: &mut Frame, app: &mut App) {
    let area = frame
        .area()
        .centered(Constraint::Percentage(60), Constraint::Length(3));

    let popup = Block::bordered()
        .title_alignment(Alignment::Center)
        .title_top("Confirm Action")
        .title_bottom("[y] Yes [n] No");

    frame.render_widget(popup, area);
}
