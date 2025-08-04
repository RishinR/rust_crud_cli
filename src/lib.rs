use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}

impl Todo {
    pub fn new(id: u32, title: String, description: String, is_completed: bool) -> Self {
        Todo {
            id,
            title,
            description,
            is_completed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TodoList {
    pub todo_list: HashMap<u32, Todo>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            todo_list: HashMap::new(),
        }
    }

    pub fn add_todo(&mut self, todo: Todo) -> Result<(), &'static str> {
        self.todo_list.insert(todo.id, todo);
        Ok(())
    }

    pub fn get_todo(&self, todo_id: u32) -> Result<&Todo, &'static str> {
        if let Some(todo) = self.todo_list.get(&todo_id) {
            return Ok(todo);
        }
        Err("No such todo is avaiable!")
    }

    pub fn update_todo(&mut self, todo_id: u32, new_todo: Todo) -> Result<(), &'static str> {
        if let Some(todo) = self.todo_list.get_mut(&todo_id) {
            *todo = new_todo;
            return Ok(());
        }
        Err("Error in updating the todo!")
    }
    pub fn remove_todo(&mut self, todo_id: u32) -> Result<(), &'static str> {
        if let Some(todo) = self.todo_list.remove(&todo_id) {
            return Ok(());
        }
        Err("Error in removing the todo!")
    }

    pub fn return_all_todos(&self) -> &HashMap<u32, Todo> {
        &self.todo_list
    }

    pub fn return_completed_todos(&self) -> HashMap<u32, Todo> {
        self.todo_list
            .iter()
            .filter(|(_, todo)| todo.is_completed)
            .map(|(id, todo)| (*id, todo.clone()))
            .collect()
    }

    pub fn return_incomplete_todos(&self) -> HashMap<u32, Todo> {
        self.todo_list
            .iter()
            .filter(|(_, todo)| !todo.is_completed)
            .map(|(id, todo)| (*id, todo.clone()))
            .collect()
    }
}
