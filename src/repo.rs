use std::{
    env::current_dir,
    fs::{create_dir, remove_dir_all, File},
    path::PathBuf,
};

use crate::{
    messages::{self, INVALID_PARAMETER_MESSAGE},
    CreateCommand, DeleteCommand,
};

pub fn create_repo(command: CreateCommand) {
    let path = match validate_path(command.path) {
        Ok(path) => path,
        Err(_) => return,
    };
    let is_forced = command.is_forced.unwrap_or(false);
    let initialize = command.inilialize.unwrap_or(false);

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

    if initialize {
        update_repo();
    }
}

pub fn delete_repo(command: DeleteCommand) {
    let path = match validate_path(command.path) {
        Ok(path) => path,
        Err(_) => return,
    };
    let is_forced = command.is_forced.unwrap_or(false);

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

pub fn update_repo() {
    println!("update or something");
}

fn validate_path(path: Option<String>) -> Result<PathBuf, String> {
    match path {
        Some(given_path) => {
            let path = PathBuf::from(&given_path);
            if path.exists() {
                Result::Ok(path)
            } else {
                println!("{}", messages::INVALID_PARAMETER_MESSAGE);
                Result::Err(INVALID_PARAMETER_MESSAGE.to_string())
            }
        }
        None => match current_dir() {
            Ok(path_buf) => Result::Ok(path_buf),
            Result::Err(err_message) => {
                println!("{}", err_message);
                Result::Err(err_message.to_string())
            }
        },
    }
}
