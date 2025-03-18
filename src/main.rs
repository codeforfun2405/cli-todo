use clap::Parser;
use cli_todo::{command::Command, handler::CmdHandler};

fn main() {
    println!("Hello, world!");

    let cmd = Command::parse();
    println!("pared command: {:?}", cmd);
    handle_cmd(cmd)
}

fn handle_cmd(cmd: Command) {
    let mut handler = CmdHandler::new(cmd.cmd);
    let res = handler.handle();
    match res {
        Ok(v) => match v {
            Some(t) => println!("todo: {:?}", t),
            None => println!("todo not found"),
        },
        Err(e) => println!("handle cmd error: {}", e),
    }
}
