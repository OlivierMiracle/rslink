use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::ops::Add;
use std::path::PathBuf;

use crate::messages;
use crate::parser::Argument;
use crate::parser::ArgumentPackage;
use crate::repo;

use walkdir::WalkDir;

pub fn add_flags() -> Vec<Argument> {
    let mut file_arg = Argument::file();
    file_arg.is_required = false;

    let args: Vec<Argument> = vec![Argument::path(), file_arg, Argument::all()];

    args
}

pub fn add<'a>(args: ArgumentPackage) -> Result<&'a str, &'a str> {
    let path = args.path.unwrap();
    let path_buf = PathBuf::from(&path);
    if !repo::is_in_repo(&path_buf) {
        return Err(messages::REPO_NOT_FOUND_MESSAGE);
    }

    if !args.all {
        let file_path = match repo::validate_path(&args.file, true) {
            Ok(file_path) => file_path,
            Err(_) => return Err(messages::INVALID_PARAMETER_MESSAGE),
        };

        let file_path = match file_path.strip_prefix(&path) {
            Ok(path) => path.to_path_buf(),
            Err(_) => return Err(messages::IMPOSSIBLE_ERROR_MESSAGE),
        };

        add_file_to_linked(path_buf, file_path)
    } else {
        for entry in WalkDir::new(&path_buf)
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

            let entry_path = match entry.path().strip_prefix(&path) {
                Ok(path) => path.to_path_buf(),
                Err(_) => return Err(messages::IMPOSSIBLE_ERROR_MESSAGE),
            };

            println!("{}", entry_path.display());
            match add_file_to_linked(path_buf.clone(), entry_path.to_path_buf()) {
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
        if row.trim() == path_entry.trim() {
            return Err(messages::ALREADY_LINKED_MESSAGE);
        }
    }

    let line = path_entry.as_str();
    let buf = line.as_bytes();

    match linked_file.write(buf) {
        Ok(_) => (),
        Err(_) => return Err(messages::FILE_WRITE_ERROR_MESSAGE),
    }

    Ok(messages::SUCCESSFUL_MESSAGE)
}
