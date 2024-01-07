use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::ops::Add;
use std::path::PathBuf;

use crate::messages;
use crate::repo;
use crate::Command;

use walkdir::WalkDir;

#[derive(Default)]
pub struct AddCommand {
    path: Option<String>,
    file_path: Option<String>,
    is_all: bool,
}

pub fn add_parse(args: Vec<String>) -> Command {
    let mut add_command = AddCommand {
        ..Default::default()
    };

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
                    add_command.path = Option::Some(args[&c + 1].to_string());
                    c += 1usize;
                } else {
                    println!("No path given in path argument LOL");
                }
            }
            "-f" | "--file" => {
                if c + 1 < *c_max {
                    add_command.file_path = Option::Some(args[&c + 1].to_string());
                    c += 1usize;
                } else {
                    println!("What file should I add? ;-;");
                }
            }
            "-a" | "--all" => add_command.is_all = true,
            _ => return Command::Help(messages::INVALID_PARAMETER_MESSAGE.to_string()),
        }

        c += 1usize;
    }

    Command::Add(add_command)
}

pub fn add<'a>(command: AddCommand) -> Result<&'a str, &'a str> {
    let path = match repo::validate_path(command.path, false) {
        Ok(path) => path,
        Err(_) => return Err(messages::INVALID_PARAMETER_MESSAGE),
    };

    if !command.is_all {
        let file_path = match repo::validate_path(command.file_path, true) {
            Ok(file_path) => file_path,
            Err(_) => return Err(messages::INVALID_PARAMETER_MESSAGE),
        };

        add_file_to_linked(path, file_path)
    } else {
        for entry in WalkDir::new(&path)
            .into_iter()
            .filter_entry(|e| !repo::is_rslink(e))
            .filter_map(|e| e.ok())
        {
            if repo::is_rslink(&entry) {
                continue;
            }
            if !entry.path().metadata().unwrap().is_file() {
                continue;
            }

            println!("{}", entry.path().display());
            match add_file_to_linked(path.clone(), entry.into_path().to_path_buf()) {
                Ok(_) => (),
                Err(message) => println!("{}", message),
            }
        }
        Ok(messages::SUCCESSFUL_MESSAGE)
    }
}

fn add_file_to_linked<'a>(path: PathBuf, file_path: PathBuf) -> Result<&'a str, &'a str> {
    let mut linked_path = path.clone();
    linked_path.push(".rslink/linked.txt");
    dbg!(&linked_path);
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

    for row in linked_contents.lines() {
        let col: Vec<&str> = row.split(' ').collect();

        if col[1].trim() == path_entry.trim() {
            return Err(messages::ALREADY_LINKED_MESSAGE);
        }
    }

    let line = "link ".to_string() + path_entry.as_str();
    let buf = line.as_bytes();

    match linked_file.write(buf) {
        Ok(_) => (),
        Err(_) => return Err(messages::FILE_WRITE_ERROR_MESSAGE),
    }

    Ok(messages::HELP_MESSAGE)
}
