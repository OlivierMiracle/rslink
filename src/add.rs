use std::fs::OpenOptions;
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
    add_hidden: bool,
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
            "-h" | "--hidden" => add_command.add_hidden = true,
            _ => return Command::Help(messages::INVALID_PARAMETER_MESSAGE.to_string()),
        }

        c += 1usize;
    }

    Command::Add(add_command)
}

pub fn add(command: AddCommand) {
    let path = match repo::validate_path(command.path, false) {
        Ok(path) => path,
        Err(_) => return,
    };

    if !command.is_all {
        let file_path = match repo::validate_path(command.file_path, true) {
            Ok(file_path) => file_path,
            Err(_) => return,
        };

        let is_file_hidden = match repo::is_hidden(&file_path) {
            Ok(is_hidden) => is_hidden,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };

        if is_file_hidden && !command.add_hidden {
            return;
        }

        add_file_to_linked(path, file_path);
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

            let is_file_hidden = match repo::is_hidden(&entry.clone().into_path().to_path_buf()) {
                Ok(is_hidden) => is_hidden,
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            };

            if is_file_hidden && !command.add_hidden {
                return;
            }

            println!("{}", entry.path().display());
            add_file_to_linked(path.clone(), entry.into_path().to_path_buf());
        }
    }
}

fn add_file_to_linked(path: PathBuf, file_path: PathBuf) {
    let mut linked_path = path.clone();
    linked_path.push(".rslink/linked.txt");
    dbg!(&linked_path);
    let mut linked_file = match OpenOptions::new()
        .write(true)
        .append(true)
        .open(linked_path)
    {
        Ok(file) => file,
        Err(err) => {
            eprintln!("{}", err.to_string());
            return;
        }
    };

    let line;

    let buf = match file_path.to_str() {
        Some(path) => {
            line = path.to_string().add("\n");
            line.as_bytes()
        }
        None => {
            return;
        }
    };

    linked_file.write(buf);
}
