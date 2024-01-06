use crate::messages;
use crate::repo;
use crate::Command;

use std::fs::{create_dir, remove_dir_all, File};

#[derive(Default)]
pub struct CreateCommand {
    path: Option<String>,
    is_forced: bool,
}

pub fn create_parse(args: Vec<String>) -> Command {
    let mut create_command: CreateCommand = CreateCommand {
        ..Default::default()
    };
    let c_max = &args.len();
    let mut c: usize = 2usize;
    loop {
        if &c >= c_max {
            break;
        }

        let arg = &args[c];
        dbg!(&arg);

        match arg.as_str() {
            "-f" | "--force" => create_command.is_forced = true,
            "-p" | "--path" => {
                if c + 1 < *c_max {
                    create_command.path = Option::Some(args[&c + 1].to_string());
                    c += 1usize;
                } else {
                    println!("No path given in path argument LOL.");
                }
            }
            _ => return Command::Help(messages::INVALID_PARAMETER_MESSAGE.to_string()),
        }

        c += 1usize;
    }

    Command::Create(create_command)
}

pub fn create_repo(command: CreateCommand) {
    let path = match repo::validate_path(command.path, false) {
        Ok(path) => path,
        Err(_) => return,
    };
    let is_forced = command.is_forced;

    let mut rslink_path = path.clone();
    rslink_path.push(".rslink/");
    let dir_exists: bool = rslink_path.exists();

    // Creation of the repo directory and files
    //
    //

    if dir_exists && !is_forced {
        println!("{}", messages::REPO_ALREADY_EXISTS_MESSAGE);
        return;
    } else if dir_exists {
        match remove_dir_all(&rslink_path) {
            Ok(_) => (),
            Err(err) => {
                println!("{}", err);
                return;
            }
        };
    }

    match create_dir(&rslink_path) {
        Ok(_) => (),
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    rslink_path.push("linked.txt");
    match File::create(&rslink_path) {
        Ok(_) => (),
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    rslink_path.pop();

    rslink_path.push("destinations.txt");
    match File::create(&rslink_path) {
        Ok(_) => (),
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    rslink_path.pop();
}
