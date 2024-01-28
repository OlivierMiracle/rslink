mod flags;
mod messages;
mod parser;
mod repo;

mod add;
mod create;
mod delete;
mod ignore;
mod remove;
mod update;

use std::env;

use crate::parser::{assign_args, parse_arguments};

fn main() {
    let args: Vec<String> = env::args().collect();

    if (*args).len() == 1usize {
        println!("Not enough arguments. Run {} help for help.", &args[0]);
        return;
    }

    let flags;

    let command: Command = match args[1].as_str() {
        "create" => {
            flags = create::create_flags();
            Command::Create
        }
        "delete" => {
            flags = delete::delete_flags();
            Command::Delete
        }
        "update" => {
            flags = update::update_flags();
            Command::Update
        }
        "add" => {
            flags = add::add_flags();
            Command::Add
        }
        "remove" => {
            flags = remove::remove_flags();
            Command::Remove
        }
        _ => {
            flags = vec![];
            Command::Help
        }
    };

    let parsed_args = match parse_arguments(args, flags) {
        Ok(args) => args,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let command_package = match assign_args(parsed_args) {
        Ok(package) => package,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    dbg!(&command_package);

    match command {
        Command::Help => println!("{}", messages::HELP_MESSAGE),
        Command::Create => create::create_repo(command_package),
        Command::Delete => delete::delete_repo(command_package),
        Command::Update => result_print(update::update_repo(command_package)),
        Command::Add => result_print(add::add(command_package)),
        Command::Remove => result_print(remove::remove(command_package)),
        _ => println!("{}", messages::HELP_MESSAGE),
    }
}

fn result_print(result: Result<&str, &str>) {
    match result {
        Ok(message) => println!("{}", message),
        Err(message) => eprintln!("{}", message),
    }
}

enum Command {
    Help,
    Create,
    Delete,
    Update,
    Status,
    Add,
    Remove,
    AddDestination,
}
