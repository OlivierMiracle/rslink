use std::path::PathBuf;

use crate::messages;
use crate::parser::Argument;
use crate::parser::ArgumentPackage;
use crate::repo;
use crate::repo::RepoFile;

use std::os::unix::fs;
use walkdir::WalkDir;

#[derive(Debug)]
enum SymlinkAction {
    Delete(String),
    Create(String, String),
}

pub fn update_flags() -> Vec<Argument> {
    let args: Vec<Argument> = vec![Argument::path()];

    args
}

pub fn update_repo<'a>(args: ArgumentPackage) -> Result<&'a str, &'a str> {
    let path = &args.path.unwrap();
    let path_buf = PathBuf::from(&path);
    if !repo::is_in_repo(&path_buf) {
        return Err(messages::REPO_NOT_FOUND_MESSAGE);
    }

    let destinations = match repo::read_repo_file(&path_buf, RepoFile::Destinations) {
        Ok(dests) => dests,
        Err(err) => return Err(err),
    };

    let mut linked = match repo::read_repo_file(&path_buf, RepoFile::Linked) {
        Ok(linked) => linked,
        Err(err) => return Err(err),
    };

    for dest in destinations {
        for action in compare_contents(&mut linked, &path, &dest) {
            match execute_action(action) {
                Ok(_) => continue,
                Err(err) => return Err(err),
            };
        }
    }
    Ok(messages::SUCCESSFUL_MESSAGE)
}

fn compare_contents(
    linked_files: &mut Vec<String>,
    source_path: &String,
    dest_path: &String,
) -> Vec<SymlinkAction> {
    let mut actions: Vec<SymlinkAction> = vec![];
    let dest = PathBuf::from(dest_path);

    dbg!(&linked_files);
    'entry_loop: for entry in WalkDir::new(&dest).into_iter().filter_map(|e| e.ok()) {
        if repo::is_rslink(&entry) {
            continue;
        }
        if !entry.path().metadata().unwrap().is_file() {
            continue;
        }

        let entry_path = entry.path().to_str().unwrap().to_string();

        dbg!(&entry_path);

        'linked_loop: for i in (0..(linked_files.len() - 1)).rev() {
            let if_equal = entry_path.ends_with(linked_files[i].as_str());

            if if_equal {
                linked_files.pop();
                continue 'entry_loop;
            }
        }
        actions.push(SymlinkAction::Delete(entry_path.clone()));
    }

    for linked in linked_files {
        actions.push(SymlinkAction::Create(
            source_path.clone() + linked.as_str(),
            dest_path.clone() + linked.as_str(),
        ));
    }

    dbg!(&actions);

    actions
}

fn execute_action<'a>(action: SymlinkAction) -> Result<(), &'a str> {
    match action {
        SymlinkAction::Create(source, destination) => match fs::symlink(source, destination) {
            Ok(_) => Ok(()),
            Err(_) => Err(messages::DESTINATION_UPDATE_ERROR_MESSAGE),
        },
        SymlinkAction::Delete(destination) => match std::fs::remove_file(destination) {
            Ok(_) => Ok(()),
            Err(_) => Err(messages::DESTINATION_UPDATE_ERROR_MESSAGE),
        },
    }
}
