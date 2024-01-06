use std::{env::current_dir, ffi::OsStr, path::PathBuf};

use walkdir::DirEntry;

use crate::messages::{self, INVALID_PARAMETER_MESSAGE};

pub fn is_rslink(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with(".rslink"))
        .unwrap_or(false)
}

pub fn is_hidden(entry: &PathBuf) -> Result<bool, &str> {
    match entry.file_name() {
        Some(entry) => Ok(entry.to_str().map(|s| s.starts_with(".")).unwrap_or(false)),
        None => return Err(messages::NOT_A_FILE_MESSAGE),
    }
}

pub fn validate_path(path: Option<String>, is_required: bool) -> Result<PathBuf, String> {
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
