use crate::storage::{get_todos_from_file, update_file};
use serde::Serialize;
use serde_derive::Deserialize;

use ratatui::widgets::ListState;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Todo {
    pub content: String,
    pub is_done: bool,
    pub id: usize,
}

impl Todo {
    fn update_content(&mut self, new_content: &str) {
        self.content.clear();
        self.content.push_str(new_content);
    }

    fn toggle_done(&mut self) {
        self.is_done = !self.is_done;
    }
}

#[derive(PartialEq)]
pub enum CurrentScreen {
    Main,
    Adding,
    Editing,
}

pub struct App {
    pub todos: Vec<Todo>,
    pub current_screen: CurrentScreen,
    pub state: ListState,
    pub add_todo_input: String,
    pub edit_todo_input: String,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> color_eyre::Result<Self> {
        let todos = get_todos_from_file()?;

        Ok(Self {
            todos,
            current_screen: CurrentScreen::Main,
            state: ListState::default().with_selected(Some(0)),
            add_todo_input: String::new(),
            edit_todo_input: String::new(),
            should_quit: false,
        })
    }

    pub fn select_next_item(&mut self) {
        self.state.select_next();
    }

    pub fn select_previous_item(&mut self) {
        self.state.select_previous();
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn add_todo(&mut self) -> color_eyre::Result<()> {
        let new_todo_id = match self.todos.last() {
            Some(last_todo) => last_todo.id + 1,
            None => 0,
        };

        self.todos.push(Todo {
            content: std::mem::take(&mut self.add_todo_input),
            is_done: false,
            id: new_todo_id,
        });

        self.state.select_last();

        update_file(&self.todos)?;

        Ok(())
    }

    pub fn remove_todo(&mut self) -> color_eyre::eyre::Result<()> {
        if let Some(selected_todo_index) = self.state.selected() {
            self.todos.remove(selected_todo_index);

            update_file(&self.todos)?;
        }

        Ok(())
    }

    pub fn edit_todo(&mut self) -> color_eyre::eyre::Result<()> {
        if let Some(selected_todo_index) = self.state.selected()
            && let Some(selected_todo) = self.todos.get_mut(selected_todo_index)
        {
            selected_todo.update_content(&self.edit_todo_input);

            update_file(&self.todos)?;
        }

        Ok(())
    }

    pub fn toggle_done(&mut self) -> color_eyre::eyre::Result<()> {
        if let Some(selected_todo_index) = self.state.selected()
            && let Some(selected_todo) = self.todos.get_mut(selected_todo_index)
        {
            selected_todo.toggle_done();

            update_file(&self.todos)?;
        }

        Ok(())
    }
}
