use ratatui::widgets::ListState;

use crate::{
    models::{Group, Todo},
    storage::Storage,
};

pub struct TodoManager {
    pub todos: Vec<Todo>,
    pub groups: Vec<Group>,
    storage: Box<dyn Storage>,
}

impl TodoManager {
    pub fn new(storage: Box<dyn Storage>) -> color_eyre::Result<Self> {
        let todos = storage.get_todos()?;
        let groups = storage.get_groups()?;
        Ok(Self {
            todos,
            groups,
            storage,
        })
    }

    pub fn create_todo(&mut self, content: &str) -> color_eyre::Result<()> {
        let new_todo_id = match self.todos.last() {
            Some(last_todo) => last_todo.id + 1,
            None => 0,
        };

        self.todos.push(Todo {
            content: content.to_string(),
            is_done: false,
            id: new_todo_id,
        });

        self.storage.save(&self.todos, &self.groups)?;

        Ok(())
    }

    pub fn delete_todo(&mut self, list_state: &ListState) -> color_eyre::eyre::Result<()> {
        if let Some(selected_todo_index) = list_state.selected() {
            self.todos.remove(selected_todo_index);

            self.storage.save(&self.todos, &self.groups)?;
        }

        Ok(())
    }

    pub fn edit_todo(
        &mut self,
        content: &str,
        list_state: &ListState,
    ) -> color_eyre::eyre::Result<()> {
        if let Some(selected_todo_index) = list_state.selected()
            && let Some(selected_todo) = self.todos.get_mut(selected_todo_index)
        {
            selected_todo.update_content(content);

            self.storage.save(&self.todos, &self.groups)?;
        }

        Ok(())
    }

    pub fn toggle_done_todo(&mut self, list_state: &ListState) -> color_eyre::eyre::Result<()> {
        if let Some(selected_todo_index) = list_state.selected()
            && let Some(selected_todo) = self.todos.get_mut(selected_todo_index)
        {
            selected_todo.toggle_done();

            self.storage.save(&self.todos, &self.groups)?;
        }

        Ok(())
    }

    pub fn create_group(&mut self, name: &str) -> color_eyre::Result<()> {
        let new_group_id = match self.groups.last() {
            Some(last_group) => last_group.id + 1,
            None => 0,
        };

        self.groups.push(Group {
            id: new_group_id,
            name: name.to_string(),
            is_collapsed: false,
        });

        self.storage.save(&self.todos, &self.groups)?;

        Ok(())
    }
}
