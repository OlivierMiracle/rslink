use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::ops::Add;

use std::path::Path;
use std::{env::current_dir, path::PathBuf};

use walkdir::DirEntry;

use crate::messages::{self, INVALID_PARAMETER_MESSAGE};

pub enum RepoFile {
    Linked,
    Destinations,
}

pub fn is_in_repo(path: &PathBuf) -> bool {
    if path.is_dir() {
        let mut rs_path = path.clone();
        rs_path.push(".rslink/");
        rs_path.try_exists().unwrap_or(false)
    } else {
        false
    }
}

pub fn is_rslink(entry: &Path) -> bool {
    let path = entry.to_str().unwrap().to_string();
    path.contains("/.rslink/") || path.ends_with("/.rslink")
}

pub fn validate_path(path: &Option<String>, is_required: bool) -> Result<PathBuf, String> {
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
        None => {
            if !is_required {
                match current_dir() {
                    Ok(path_buf) => Result::Ok(path_buf),
                    Result::Err(err_message) => {
                        println!("{}", err_message);
                        Result::Err(err_message.to_string())
                    }
                }
            } else {
                Result::Err(messages::LACKING_PARAMETER_MESSAGE.to_string())
            }
        }
    }
}

pub fn ensure_backslash(dir_path: String) -> String {
    if !dir_path.ends_with('/') {
        dir_path.add("/")
    } else {
        dir_path
    }
}

pub fn read_repo_file<'a>(path: &PathBuf, repo_file: RepoFile) -> Result<Vec<String>, &'a str> {
    let mut linked_path = path.clone();
    let relative_path = match repo_file {
        RepoFile::Linked => ".rslink/linked.txt",
        RepoFile::Destinations => ".rslink/destinations.txt",
    };
    linked_path.push(relative_path);

    let mut linked_file = match OpenOptions::new().read(true).open(linked_path) {
        Ok(file) => file,
        Err(_) => {
            return Err(messages::FILE_OPEN_ERROR_MESSAGE);
        }
    };

    let mut linked_contents: String = "".to_string();
    match linked_file.read_to_string(&mut linked_contents) {
        Ok(_) => (),
        Err(_) => return Err(messages::FILE_READ_ERROR_MESSAGE),
    }

    let mut rows: Vec<String> = vec![];

    for row in linked_contents.lines() {
        rows.push(row.to_string());
    }

    Ok(rows)
}

pub fn add_to_repo_file<'a>(
    path: PathBuf,
    path_to_add: PathBuf,
    repo_file: RepoFile,
) -> Result<&'a str, &'a str> {
    let mut linked_path = path.clone();
    let relative_path = match repo_file {
        RepoFile::Linked => ".rslink/linked.txt",
        RepoFile::Destinations => ".rslink/destinations.txt",
    };
    linked_path.push(relative_path);

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

    let path_entry = match path_to_add.to_str() {
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_is_rslink() {
        assert!(is_rslink(Path::new(
            "/home/olivermiracle/.rslinkTestRepo/.rslink/"
        )));
        assert!(is_rslink(Path::new(
            "/home/olivermiracle/.rslinkTestRepo/.rslink/linked.txt"
        )));
        assert!(is_rslink(Path::new(
            "/home/olivermiracle/.rslinkTestRepo/.rslink"
        )));

        assert!(!is_rslink(Path::new(
            "/home/olivermiracle/.rslinkTestRepo/"
        )));
    }
}
