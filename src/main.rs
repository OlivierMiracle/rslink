mod linker;
mod messages;
mod repo;

use std::{env, usize};

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

                match arg.as_str() {
                    "-f" | "--force" => create_command.is_forced = Option::Some(true),
                    "-p" | "--path" => {
                        if c + 1 < *c_max {
                            create_command.path = Option::Some(args[&c + 1].to_string());
                        } else {
                            println!("No path given in path argument LOL.");
                        }
                    }
                    _ => {
                        println!("Invalid parameter. Next time try better.");
                        return;
                    }
                }

                c += 1usize;
            }
            Command::Create(create_command)
        }
        _ => Command::Help(messages::UNKNOWN_COMMAND_MESSAGE.to_string()),
    };

    match command {
        Command::Help(message) => println!("{}", message),
        _ => println!(),
    }
}

enum Command {
    Help(String),
    Create(CreateCommand),
    Status(StatusCommand),
    AddDestination(),
    Update(),
}

#[derive(Default)]
struct CreateCommand {
    path: Option<String>,
    is_forced: Option<bool>,
}

struct StatusCommand {
    path: Option<String>,
    if_destinations: Option<bool>,
    if_added: Option<bool>,
    if_deleted: Option<bool>,
}
