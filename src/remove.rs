use crate::messages;
use crate::repo;
use crate::Command;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::ops::Add;

#[derive(Default)]
pub struct RemoveCommand {
    path: Option<String>,
    file_path: Option<String>,
}

pub fn remove_parse(args: Vec<String>) -> Command {
    let mut remove_command = RemoveCommand::default();

    let c_max = &args.len();
    let mut c: usize = 2usize;
    loop {
        if &c >= c_max {
            break;
        }

        let arg = &args[c];

        match arg.as_str() {
            "-p" | "--path" => {
                if c + 1 < *c_max {
                    remove_command.path = Option::Some(args[&c + 1].to_string());
                    c += 1usize;
                } else {
                    println!("No path given in path argument LOL");
                }
            }
            "-f" | "--file" => {
                if c + 1 < *c_max {
                    remove_command.file_path = Option::Some(args[&c + 1].to_string());
                    c += 1usize;
                } else {
                    println!("What file should I add? ;-;");
                }
            }
            _ => return Command::Help(messages::INVALID_PARAMETER_MESSAGE.to_string()),
        }

        c += 1usize;
    }

    Command::Remove(remove_command)
}

pub fn remove<'a>(command: RemoveCommand) -> Result<&'a str, &'a str> {
    let path = match repo::validate_path(command.path, false) {
        Ok(path) => path,
        Err(_) => return Err(messages::INVALID_PARAMETER_MESSAGE),
    };

    if !repo::is_in_repo(&path) {
        return Err(messages::REPO_NOT_FOUND_MESSAGE);
    }

    let file_path = match repo::validate_path(command.file_path, true) {
        Ok(file_path) => file_path,
        Err(_) => return Err(messages::INVALID_PARAMETER_MESSAGE),
    };

    let mut linked_path = path.clone();
    linked_path.push(".rslink/linked.txt");

    let mut linked_file = match OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(linked_path)
    {
        Ok(file) => file,
        Err(_) => {
            return Err(messages::FILE_OPEN_ERROR_MESSAGE);
        }
    };

    let path_entry = match file_path.to_str() {
        Some(path) => path.to_string().add("\n"),
        None => return Err(messages::INVALID_PARAMETER_MESSAGE),
    };

    let mut linked_contents: String = "".to_string();

    match linked_file.read_to_string(&mut linked_contents) {
        Ok(_) => (),
        Err(_) => return Err(messages::FILE_READ_ERROR_MESSAGE),
    }

    linked_file.set_len(0);

    let mut counter = 0;

    for row in linked_contents.lines() {
        let col: Vec<&str> = row.split(' ').collect();

        if col[1].trim() == path_entry.trim() {
            println!("{}", row);
            counter += 1;
            continue;
        }
        let row = row.to_string().add("\n");
        let buf = row.as_bytes();

        linked_file.write(buf);
    }

    println!(
        "{}",
        "Files removed: ".to_string() + counter.to_string().as_str()
    );
    Ok(messages::SUCCESSFUL_MESSAGE)
}
