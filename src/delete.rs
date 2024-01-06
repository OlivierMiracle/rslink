use std::fs::remove_dir_all;

use crate::messages;
use crate::repo;
use crate::Command;

#[derive(Default)]
pub struct DeleteCommand {
    path: Option<String>,
    is_forced: bool,
}

pub fn delete_parse(args: Vec<String>) -> Command {
    let mut delete_command: DeleteCommand = DeleteCommand {
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
            "-f" | "--force" => delete_command.is_forced = true,
            "-p" | "--path" => {
                if c + 1 < *c_max {
                    delete_command.path = Option::Some(args[&c + 1].to_string());
                    c += 1usize;
                } else {
                    println!("No path given in path argument LOL.");
                }
            }
            _ => return Command::Help(messages::INVALID_PARAMETER_MESSAGE.to_string()),
        }
        c += 1usize;
    }

    Command::Delete(delete_command)
}

pub fn delete_repo(command: DeleteCommand) {
    let path = match repo::validate_path(command.path, false) {
        Ok(path) => path,
        Err(_) => return,
    };
    let is_forced = command.is_forced;

    let mut rslink_dir = path.clone();
    rslink_dir.push(".rslink/");
    let dir_exists: bool = rslink_dir.exists();

    // Deletion of the repo directory and files
    //
    //

    if !dir_exists && !is_forced {
        println!("{}", messages::REPO_DOES_NOT_EXIST_MESSAGE);
    } else if dir_exists {
        match remove_dir_all(&rslink_dir) {
            Ok(_) => (),
            Err(err) => {
                println!("{}", err);
                return;
            }
        };
    }
}
