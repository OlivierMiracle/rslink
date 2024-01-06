mod linker;
mod messages;
mod repo;

mod add;
mod create;
mod delete;
mod update;

use std::env;

use crate::add::AddCommand;
use crate::create::CreateCommand;
use crate::delete::DeleteCommand;
use crate::update::UpdateCommand;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if (*args).len() == 1usize {
        println!("Not enough arguments. Run {} help for help.", &args[0]);
        return;
    }

    let command: Command = match args[1].as_str() {
        "help" => Command::Help(messages::HELP_MESSAGE.to_string()),
        "create" => create::create_parse(args),
        "delete" => delete::delete_parse(args),
        "update" => update::update_parse(args),
        "add" => add::add_parse(args),
        _ => Command::Help(messages::UNKNOWN_COMMAND_MESSAGE.to_string()),
    };

    match command {
        Command::Help(message) => println!("{}", message),
        Command::Create(create_command) => create::create_repo(create_command),
        Command::Delete(delete_command) => delete::delete_repo(delete_command),
        Command::Update(update_command) => update::update_repo(update_command),
        Command::Add(add_command) => add::add(add_command),
        _ => println!(),
    }
}

enum Command {
    Help(String),
    Create(CreateCommand),
    Delete(DeleteCommand),
    Update(UpdateCommand),
    Status(StatusCommand),
    Add(AddCommand),
    AddDestination(),
}

struct StatusCommand {
    path: Option<String>,
    if_destinations: bool,
    if_added: bool,
    if_deleted: bool,
}
