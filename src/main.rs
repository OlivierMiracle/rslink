mod linker;
mod messages;
mod repo;

use std::{env, usize};

use crate::repo::delete_repo;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if (*args).len() == 1usize {
        println!("Not enough arguments. Run {} help for help.", &args[0]);
        return;
    }

    let command: Command = match args[1].as_str() {
        "help" => Command::Help(messages::HELP_MESSAGE.to_string()),
        "create" => {
            let mut create_command: CreateCommand = CreateCommand {
                ..Default::default()
            };
            let c_max = &args.len();
            let mut c: usize = 2usize;
            loop {
                if !(&c < c_max) {
                    break;
                }

                let arg = &args[c];
                dbg!(&arg);

                match arg.as_str() {
                    "-f" | "--force" => create_command.is_forced = Option::Some(true),
                    "-i" | "--initialize" => create_command.inilialize = Option::Some(true),
                    "-p" | "--path" => {
                        if c + 1 < *c_max {
                            create_command.path = Option::Some(args[&c + 1].to_string());
                            c += 1usize;
                        } else {
                            println!("No path given in path argument LOL.");
                        }
                    }
                    _ => {
                        println!("{}", messages::INVALID_PARAMETER_MESSAGE);
                        return;
                    }
                }

                c += 1usize;
            }

            Command::Create(create_command)
        }
        "delete" => {
            let mut delete_command: DeleteCommand = DeleteCommand {
                ..Default::default()
            };

            let c_max = &args.len();
            let mut c: usize = 2usize;
            loop {
                if !(&c < c_max) {
                    break;
                }

                let arg = &args[c];
                dbg!(&arg);

                match arg.as_str() {
                    "-f" | "--force" => delete_command.is_forced = Option::Some(true),
                    "-p" | "--path" => {
                        if c + 1 < *c_max {
                            delete_command.path = Option::Some(args[&c + 1].to_string());
                            c += 1usize;
                        } else {
                            println!("No path given in path argument LOL.");
                        }
                    }
                    _ => {
                        println!("{}", messages::INVALID_PARAMETER_MESSAGE);
                        return;
                    }
                }

                c += 1usize;
            }

            Command::Delete(delete_command)
        }
        "update" => Command::Update(),
        _ => Command::Help(messages::UNKNOWN_COMMAND_MESSAGE.to_string()),
    };

    match command {
        Command::Help(message) => println!("{}", message),
        Command::Create(create_command) => repo::create_repo(create_command),
        Command::Delete(delete_command) => repo::delete_repo(delete_command),
        Command::Update() => repo::update_repo(),
        _ => println!(),
    }
}

enum Command {
    Help(String),
    Create(CreateCommand),
    Delete(DeleteCommand),
    Status(StatusCommand),
    AddDestination(),
    Update(),
}

#[derive(Default)]
struct CreateCommand {
    is_forced: Option<bool>,
    inilialize: Option<bool>,
    path: Option<String>,
}

#[derive(Default)]
struct DeleteCommand {
    path: Option<String>,
    is_forced: Option<bool>,
}

struct StatusCommand {
    path: Option<String>,
    if_destinations: Option<bool>,
    if_added: Option<bool>,
    if_deleted: Option<bool>,
}
