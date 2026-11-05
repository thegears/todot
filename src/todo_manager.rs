use ratatui::widgets::ListState;

use crate::{
    models::{Group, Todo, TodoGroup},
    storage::Storage,
};

pub struct TodoManager {
    pub todos: Vec<Todo>,
    pub groups: Vec<Group>,
    storage: Box<dyn Storage>,
}

#[derive(Debug)]
pub enum ItemType {
    Group(usize),
    Todo(usize),
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

    pub fn create_todo(&mut self, content: &str, group: TodoGroup) -> color_eyre::Result<()> {
        let new_todo_id = match self.todos.last() {
            Some(last_todo) => last_todo.id + 1,
            None => 0,
        };

        self.todos.push(Todo {
            content: content.to_string(),
            is_done: false,
            id: new_todo_id,
            group_id: group,
        });

        self.storage.save(&self.todos, &self.groups)?;

        Ok(())
    }

    pub fn delete_thing(&mut self, list_state: &ListState) -> color_eyre::eyre::Result<()> {
        if let Some(selected_thing_index) = list_state.selected() {
            let items = self.get_all_items();

            if let Some(selected_thing) = items.get(selected_thing_index) {
                match selected_thing {
                    ItemType::Todo(todo_id) => {
                        self.todos.retain(|t| t.id != *todo_id);
                    }
                    ItemType::Group(group_id) => {
                        self.groups.retain(|g| g.id != *group_id);
                        self.todos.retain(|t| match t.group_id {
                            TodoGroup::Some { group_id: id } => id != *group_id,
                            TodoGroup::None => true,
                        });
                    }
                }
            }

            self.storage.save(&self.todos, &self.groups)?;
        }

        Ok(())
    }

    pub fn edit_todo(
        &mut self,
        content: &str,
        list_state: &ListState,
    ) -> color_eyre::eyre::Result<()> {
        if let Some(selected_thing_index) = list_state.selected() {
            let items = self.get_all_items();

            if let Some(selected_thing) = items.get(selected_thing_index)
                && let ItemType::Todo(todo_id) = selected_thing
                && let Some(todo) = self.todos.iter_mut().find(|t| t.id == *todo_id)
            {
                todo.update_content(content);
            }

            self.storage.save(&self.todos, &self.groups)?;
        }

        Ok(())
    }

    pub fn toggle_done_todo(&mut self, list_state: &ListState) -> color_eyre::eyre::Result<()> {
        if let Some(selected_thing_index) = list_state.selected() {
            let items = self.get_all_items();

            if let Some(selected_thing) = items.get(selected_thing_index)
                && let ItemType::Todo(todo_id) = selected_thing
                && let Some(todo) = self.todos.iter_mut().find(|t| t.id == *todo_id)
            {
                todo.toggle_done();
            }

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
            is_collapsed: true,
        });

        self.storage.save(&self.todos, &self.groups)?;

        Ok(())
    }

    pub fn get_all_items(&self) -> Vec<ItemType> {
        let mut items: Vec<ItemType> = Vec::new();

        for todo in &self.todos {
            if todo.group_id == TodoGroup::None {
                items.push(ItemType::Todo(todo.id));
            }
        }

        for group in &self.groups {
            let is_collapsed = group.is_collapsed;
            let id = group.id;
            items.push(ItemType::Group(group.id));

            if !is_collapsed {
                for todo in &self.todos {
                    match todo.group_id {
                        TodoGroup::Some { group_id } => {
                            if group_id == id {
                                items.push(ItemType::Todo(todo.id));
                            }
                        }
                        TodoGroup::None => {}
                    }
                }
            }
        }

        items
    }
}
