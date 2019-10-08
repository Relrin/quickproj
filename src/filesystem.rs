use std::fs::create_dir_all;

use dirs::home_dir;

use crate::error::Error;
use std::path::PathBuf;

pub fn get_home_directory() -> Result<PathBuf, Error> {
    match home_dir() {
        Some(home_dir_path) => Ok(home_dir_path),
        None => {
            let message = "Home directory was not found. Probably unsupported platform?";
            Err(Error::Other(String::from(message)))
        }
    }
}

pub fn get_quickproj_directory() -> Result<PathBuf, Error> {
    let home_directory = get_home_directory()?;
    Ok(home_directory.join(".quickproj"))
}

pub fn get_template_directory() -> Result<PathBuf, Error> {
    let quickproj_directory = get_quickproj_directory()?;
    Ok(quickproj_directory.join("templates"))
}

pub fn create_directory(path: &PathBuf) -> Result<(), Error> {
    Ok(create_dir_all(path).unwrap())
}