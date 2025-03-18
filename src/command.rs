// define the create, update, delete command

use clap::{Args, Parser};

#[derive(Debug, Parser)]
pub struct Command {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Args, Clone)]
pub struct CreateArgs {
    pub name: String,
    pub deadline: String,
}

#[derive(Debug, Args, Clone)]
pub struct UpdateArgs {
    pub id: i64,
    pub name: String,
    pub deadline: String,
    pub status: String,
}

#[derive(Debug, Args, Clone)]
pub struct DeleteArgs {
    pub id: i64,
}

// define the create, update, delete command
#[derive(Debug, Parser, Clone)]
pub enum SubCommand {
    /// create a todo
    Create(CreateArgs),
    /// update todo name, status, or deadline
    Update(UpdateArgs),
    /// delete todo by id
    Delete(DeleteArgs),
}
