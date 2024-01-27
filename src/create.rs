use crate::parser::Argument;
use crate::{messages, parser::ArgumentPackage};

use std::fs::{create_dir, remove_dir_all, File};
use std::path::PathBuf;

pub fn create_flags() -> Vec<Argument> {
    let args: Vec<Argument> = vec![Argument::path(), Argument::forced()];

    args
}

pub fn create_repo(args: ArgumentPackage) {
    let mut rslink_path = PathBuf::from(&args.path.unwrap());
    rslink_path.push(".rslink/");
    let dir_exists: bool = rslink_path.exists();

    // Creation of the repo directory and files
    //
    //

    if dir_exists && !args.forced {
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
