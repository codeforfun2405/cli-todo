use crate::{
    command::{CreateArgs, DeleteArgs, SubCommand, UpdateArgs},
    store::{Status, Todo, TodoStore},
};

pub struct CmdHandler {
    pub cmd: SubCommand,
    pub store: TodoStore,
}

impl CmdHandler {
    pub fn new(cmd: SubCommand) -> Self {
        Self {
            cmd,
            store: TodoStore::new(1000),
        }
    }

    pub fn handle(&mut self) -> Result<Option<Todo>, String> {
        match self.cmd.clone() {
            SubCommand::Create(args) => self.handle_create(&args),
            SubCommand::Update(args) => self.handle_update(&args),
            SubCommand::Delete(args) => self.handle_delete(&args),
        }
    }

    fn handle_create(&mut self, args: &CreateArgs) -> Result<Option<Todo>, String> {
        let todo = self
            .store
            .add_todo(args.name.clone(), args.deadline.clone());
        Ok(Some(todo))
    }

    fn handle_update(&mut self, args: &UpdateArgs) -> Result<Option<Todo>, String> {
        let state = Status::from(args.status.clone());
        if state == Status::Unknown {
            return Err("unknow status".to_string());
        }

        let todo_res =
            self.store
                .update_todo(args.id, args.name.clone(), args.deadline.clone(), state);
        Ok(todo_res)
    }

    fn handle_delete(&mut self, args: &DeleteArgs) -> Result<Option<Todo>, String> {
        let todo_res = self.store.delete_todo(args.id);
        Ok(todo_res)
    }
}
