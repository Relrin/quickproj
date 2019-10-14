use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::path::PathBuf;

use dirs::home_dir;
use rm_rf::force_remove_all;
use walkdir::{DirEntry, WalkDir};

use crate::error::Error;

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

pub fn get_templates_directory() -> Result<PathBuf, Error> {
    let quickproj_directory = get_quickproj_directory()?;
    Ok(quickproj_directory.join("templates"))
}

pub fn create_directory(path: &PathBuf) -> Result<(), Error> {
    let operation_result = create_dir_all(path)?;
    Ok(operation_result)
}

pub fn basename<'a>(path: &'a str, sep: char) -> String {
    let pieces = path.rsplit(sep);
    let result: Cow<'a, str> = match pieces.clone().next() {
        Some(p) => p.into(),
        None => path.into(),
    };
    String::from(result)
}

pub fn get_repositories_map() -> Result<HashMap<String, String>, Error> {
    let directory = get_templates_directory()?;
    let mut repositories: HashMap<String, String> = HashMap::new();
    WalkDir::new(directory.clone())
        .max_depth(1)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| is_repository_directory(&directory, entry))
        .filter(|entry| !is_hidden_file(entry))
        .for_each(|entry| {
            let repository_name = entry
                .clone()
                .into_path()
                .file_name().unwrap()
                .to_str().unwrap()
                .to_string();
            let path = entry
                .clone()
                .into_path()
                .into_os_string()
                .into_string()
                .unwrap();
            repositories.insert(repository_name, path);
        });

    Ok(repositories)
}

pub fn get_templates_map() -> Result<HashMap<String, String>, Error> {
    let directory = get_templates_directory()?;
    let mut templates: HashMap<String, String> = HashMap::new();
    WalkDir::new(directory.clone())
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| is_template_directory(&directory, entry))
        .for_each(|entry| {
            let template_name = entry
                .clone()
                .into_path()
                .file_name().unwrap()
                .to_str().unwrap()
                .to_string();
            let path = entry
                .clone()
                .into_path()
                .into_os_string()
                .into_string()
                .unwrap();
            templates.insert(template_name, path);
        });

    Ok(templates)
}


pub fn is_hidden_file(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}


pub fn is_repository_directory(directory: &PathBuf, entry: &DirEntry) -> bool {
    let entry_path = entry.path();
    let is_directory = entry.file_type().is_dir();
    let is_root_directory = entry_path == directory;
    is_directory && !is_root_directory && !is_template_directory(directory, entry)
}

pub fn is_template_directory(directory: &PathBuf, entry: &DirEntry) -> bool {
    let entry_path = entry.path();
    match entry.file_type().is_dir() && entry_path != directory {
        true => {
            let config_path = entry_path.join("config.toml");
            config_path.exists()
        }
        false => false,
    }
}

pub fn delete_repository_by_name(repository_name: &String) -> Result<(), Error> {
    let templates_directory = get_templates_directory()?;
    let repository_path = templates_directory.join(repository_name);
    let operation_result = force_remove_all(repository_path)?;
    Ok(operation_result)
}

pub fn delete_template_by_path(template_path: &String) -> Result<(), Error> {
    let operation_result = force_remove_all(template_path)?;
    Ok(operation_result)
}
