use crate::messages;
use crate::repo;
use crate::Command;

pub struct UpdateCommand {
    path: Option<String>,
    destinations: bool,
    repository: bool,
}

pub fn update_parse(args: Vec<String>) -> Command {
    let mut update_command: UpdateCommand = UpdateCommand {
        path: Option::None,
        destinations: true,
        repository: true,
    };

    let c_max = &args.len();

    if c_max > &2usize {
        update_command.destinations = false;
        update_command.repository = false;

        let mut c: usize = 2usize;
        loop {
            if &c >= c_max {
                break;
            }

            let arg = &args[c];
            dbg!(&arg);

            match arg.as_str() {
                "-p" | "--path" => {
                    if c + 1 < *c_max {
                        update_command.path = Option::Some(args[&c + 1].to_string());
                        c += 1usize;
                    } else {
                        println!("No path given in path argument LOL.");
                    }
                }
                "-d" | "--destinations" => update_command.destinations = true,
                "-r" | "--repository" => update_command.repository = true,
                _ => return Command::Help(messages::INVALID_PARAMETER_MESSAGE.to_string()),
            }
            c += 1usize;
        }
    }

    Command::Update(update_command)
}

pub fn update_repo(command: UpdateCommand) {
    let path = match repo::validate_path(command.path, false) {
        Ok(path) => path,
        Err(_) => return,
    };
    let repository = command.repository;
    let destinations = command.destinations;

    if repository {}

    if destinations {}
}
