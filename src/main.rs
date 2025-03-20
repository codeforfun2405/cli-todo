use std::io::Write;

use clap::Parser;
use cli_todo::{command::Command, handler::CmdHandler, store::TodoStore};

fn main() {
    println!("Hello, world!");

    let mut todo_store = TodoStore::new(1000);

    loop {
        print!(">>>");

        std::io::stdout().flush().unwrap();
        let mut buf = String::new();
        match std::io::stdin().read_line(&mut buf) {
            Ok(_) => {
                println!("read buf: {}", buf);
                let cmd = parse_cmd(buf);
                handle_cmd(cmd, &mut todo_store)
            }
            Err(e) => println!("read line error: {:?}", e),
        }
    }
}

fn handle_cmd(cmd: Command, store: &mut TodoStore) {
    let mut handler = CmdHandler::new(cmd.cmd, store);
    let res = handler.handle();
    match res {
        Ok(v) => match v {
            Some(t) => println!("todo: {:?}", t),
            None => println!("todo not found"),
        },
        Err(e) => println!("handle cmd error: {}", e),
    }
}

fn parse_cmd(buf: String) -> Command {
    let args: Vec<&str> = buf
        .split('"')
        .filter(|s| !s.trim().is_empty() && *s != " ")
        .map(|v| v.trim())
        .collect();
    println!("args: {:?}", args);

    let mut new_args = vec!["cli-todo"];
    new_args.extend(args);

    Command::parse_from(new_args)
}
