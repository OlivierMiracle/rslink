use crate::messages;
use crate::parser::Argument;
use crate::parser::ArgumentPackage;
use crate::repo;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::ops::Add;

pub fn remove_flags() -> Vec<Argument> {
    let args: Vec<Argument> = vec![Argument::path(), Argument::file()];

    args
}

pub fn remove<'a>(args: ArgumentPackage) -> Result<&'a str, &'a str> {
    let path = match repo::validate_path(args.path, false) {
        Ok(path) => path,
        Err(_) => return Err(messages::INVALID_PARAMETER_MESSAGE),
    };

    if !repo::is_in_repo(&path) {
        return Err(messages::REPO_NOT_FOUND_MESSAGE);
    }

    let file_path = match repo::validate_path(args.file, true) {
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
