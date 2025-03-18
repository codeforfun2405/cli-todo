#[derive(Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub name: String,
    pub deadline: String,
    pub status: Status,
}

pub struct TodoStore {
    pub todos: Vec<Todo>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    PENDING,
    WIP, // working in progress
    DONE,
    Unknown,
}

impl From<String> for Status {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "Pending" => Status::PENDING,
            "wip" => Status::WIP,
            "Done" => Status::DONE,
            _ => Status::Unknown,
        }
    }
}

impl Todo {
    pub fn new(id: i64, name: String, deadline: String) -> Self {
        Self {
            id,
            name,
            deadline,
            status: Status::PENDING, // default status
        }
    }
}

impl TodoStore {
    pub fn new(cap: usize) -> Self {
        Self {
            todos: Vec::with_capacity(cap),
        }
    }

    pub fn get_by_id(&self, id: i64) -> Option<Todo> {
        let res: Vec<Todo> = self
            .todos
            .iter()
            .filter(|x| x.id == id)
            .map(|t| t.clone())
            .collect();

        if res.len() <= 0 {
            None
        } else {
            Some(res[0].clone())
        }
    }

    pub fn add_todo(&mut self, name: String, deadline: String) -> Todo {
        let max_id = self.todos.iter().map(|v| v.id).max().unwrap_or(0);
        let todo = Todo::new(max_id + 1, name, deadline);
        self.todos.push(todo.clone());
        todo
    }

    pub fn delete_todo(&mut self, id: i64) -> Option<Todo> {
        let todo_res = self.get_by_id(id);
        if todo_res.is_none() {
            return None;
        }

        self.todos.retain(|v| v.id == id);
        todo_res
    }

    pub fn update_todo(
        &mut self,
        id: i64,
        name: String,
        deadline: String,
        status: Status,
    ) -> Option<Todo> {
        let todo_res = self.get_by_id(id);
        if todo_res.is_none() {
            return None;
        }

        self.todos.iter_mut().for_each(|v| {
            if v.id == id {
                v.name = name.clone();
                v.deadline = deadline.clone();
                v.status = status.clone();
            }
        });

        todo_res
    }
}
