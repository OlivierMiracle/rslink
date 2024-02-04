use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::ops::Add;
use std::path::PathBuf;

use crate::messages;
use crate::parser::Argument;
use crate::parser::ArgumentPackage;
use crate::repo;
use crate::repo::ensure_backslash;

use walkdir::WalkDir;

pub fn add_flags() -> Vec<Argument> {
    let mut file_arg = Argument::file();
    file_arg.is_required = false;

    let args: Vec<Argument> = vec![
        Argument::path(),
        file_arg,
        Argument::all(),
        Argument::destination(),
    ];

    args
}

pub fn add<'a>(args: ArgumentPackage) -> Result<&'a str, &'a str> {
    dbg!("bobrza");

    if args.destination.is_some() {
        if args.file.is_some() {
            return Err(messages::INVALID_PARAMETER_MESSAGE);
        }

        return add_destination(args);
    } else {
        return add_file(args);
    }
}

fn add_destination<'a>(args: ArgumentPackage) -> Result<&'a str, &'a str> {
    let path = args.path.unwrap();
    let path_buf = PathBuf::from(&path);
    if !repo::is_in_repo(&path_buf) {
        return Err(messages::REPO_NOT_FOUND_MESSAGE);
    }

    let mut dest = args.destination.unwrap();
    dest = ensure_backslash(dest);

    let dest_path = match repo::validate_path(&Some(dest), true) {
        Ok(dest_path) => dest_path,
        Err(_) => return Err(messages::INVALID_PARAMETER_MESSAGE),
    };

    if !dest_path.metadata().unwrap().is_dir() {
        return Err(messages::DESTINATION_IS_NOT_A_DIRECTORY);
    }

    dbg!(&dest_path);
    repo::add_to_repo_file(path_buf, dest_path, repo::RepoFile::Destinations)
}

fn add_file<'a>(args: ArgumentPackage) -> Result<&'a str, &'a str> {
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

        repo::add_to_repo_file(path_buf, file_path, repo::RepoFile::Linked)
    } else {
        for entry in WalkDir::new(&path_buf)
            .into_iter()
            .filter_entry(|e| !repo::is_rslink(e.path()))
            .filter_map(|e| e.ok())
        {
            if repo::is_rslink(entry.path()) {
                continue;
            }
            if !entry.path().metadata().unwrap().is_file() {
                continue;
            }

            let entry_path = match entry.path().strip_prefix(&path) {
                Ok(path) => path.to_path_buf(),
                Err(_) => return Err(messages::IMPOSSIBLE_ERROR_MESSAGE),
            };

            dbg!(&entry_path);

            println!("{}", entry_path.display());
            match repo::add_to_repo_file(
                path_buf.clone(),
                entry_path.to_path_buf(),
                repo::RepoFile::Linked,
            ) {
                Ok(_) => (),
                Err(message) => println!("{}", message),
            }
        }
        Ok(messages::SUCCESSFUL_MESSAGE)
    }
}
