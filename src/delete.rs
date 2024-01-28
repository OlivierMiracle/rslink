use std::fs::remove_dir_all;

use crate::messages;
use crate::parser::{Argument, ArgumentPackage};
use crate::repo;

pub fn delete_flags() -> Vec<Argument> {
    let args: Vec<Argument> = vec![Argument::path(), Argument::forced()];

    args
}

pub fn delete_repo(args: ArgumentPackage) {
    let path = match repo::validate_path(&args.path, false) {
        Ok(path) => path,
        Err(_) => return,
    };
    let is_forced = args.forced;

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
