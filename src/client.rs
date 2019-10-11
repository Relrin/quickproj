use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

use indicatif::HumanDuration;

use crate::cli::{Command, EntityTypeEnum, InstallerTypeEnum};
use crate::constants::OPERATION_HAS_BEEN_COMPLETED_EMOJI;
use crate::error::Error;
use crate::filesystem::{
    create_directory, delete_repository, delete_template_by_path,
    get_templates_directory, get_repositories_map, get_templates_map,
    is_template_exist,
};
use crate::installers::git::GitInstaller;
use crate::installers::traits::TemplateInstaller;
use crate::plugins::{is_correct_plugins_list, Plugin};
use crate::terminal::ask_for_replacing_template;

pub struct Client {
    repositories: HashMap<String, String>,
    templates: HashMap<String, String>,
}

impl Client {
    pub fn new() -> Result<Self, Error> {
        let templates_directory = get_templates_directory()?;
        if !templates_directory.exists() {
            create_directory(&templates_directory);
        }

        let repositories_map = get_repositories_map()?;
        let templates_map = get_templates_map()?;
        Ok(Client {
            repositories: repositories_map,
            templates: templates_map,
        })
    }

    pub fn run(&self, command: &Command) {
        let result = match command {
            Command::Init { plugins, options } => self.init_project(plugins, options),
            Command::Install {
                installer_type,
                path,
                template_name,
            } => self.install_template(installer_type, path, template_name),
            Command::List { entity } => match entity {
                EntityTypeEnum::Repository => self.show_repository_list(),
                EntityTypeEnum::Template => self.show_template_list()
            },
            Command::Delete { entity, name } => match entity {
                EntityTypeEnum::Repository => self.delete_repository(&name),
                EntityTypeEnum::Template => self.delete_template(&name),
            },
        };

        match result {
            Ok(_) => {}
            Err(err) => println!("{}", err),
        }
    }

    fn init_project(&self, plugins: &Vec<String>, options: &Vec<String>) -> Result<(), Error> {
        Ok(())
    }

    fn install_template(
        &self,
        installer: &InstallerTypeEnum,
        path: &String,
        template_name: &Option<String>,
    ) -> Result<(), Error> {
        let worker = match installer {
            InstallerTypeEnum::Git => Box::new(GitInstaller::new()),
            InstallerTypeEnum::Local => Box::new(GitInstaller::new()),
        };

        let used_template_name = template_name
            .clone()
            .unwrap_or(worker.get_template_name(path)?);
        if is_template_exist(&used_template_name)? {
            ask_for_replacing_template()?;
            delete_repository(&used_template_name)?;
        }

        worker.install(path, &used_template_name)
    }

    fn show_repository_list(&self) -> Result<(), Error> {
        if self.repositories.is_empty() {
            println!("The templates folder is empty. Please, install templates first.");
            return Ok(())
        }

        let repositories = self.repositories
            .keys()
            .map(|key| &**key)
            .collect::<Vec<_>>()
            .join("\n  ");

        println!("Available repositories:\n  {}", repositories);
        Ok(())
    }

    fn show_template_list(&self) -> Result<(), Error> {
        if self.templates.is_empty() {
            println!("The templates folder is empty. Please, install templates first.");
            return Ok(())
        }

        let templates = self.templates
            .keys()
            .map(|key| &**key)
            .collect::<Vec<_>>()
            .join("\n  ");

        println!("Available templates:\n  {}", templates);
        Ok(())
    }

    fn delete_repository(&self, repository_name: &String) -> Result<(), Error> {
        if self.repositories.is_empty() {
            println!("The templates folder is empty. Please, install templates first.");
            return Ok(())
        }

        if !self.repositories.contains_key(repository_name) {
            println!("The name of the deleted repository is invalid. Please, make ensure \
                    that the template with this name exists.");
            return Ok(())
        }

        let started = Instant::now();
        let repository_path = self.repositories.get(repository_name).unwrap();
        delete_repository(repository_path)?;
        println!(
            "{} Done in {}",
            OPERATION_HAS_BEEN_COMPLETED_EMOJI,
            HumanDuration(started.elapsed())
        );
        Ok(())
    }

    fn delete_template(&self, template_name: &String) -> Result<(), Error> {
        if self.templates.is_empty() {
            println!("The templates folder is empty. Please, install templates first.");
            return Ok(())
        }

        if !self.templates.contains_key(template_name) {
            println!("The name of the deleted template is invalid. Please, make ensure \
                    that the template with this name exists.");
            return Ok(())
        }

        let started = Instant::now();
        let template_path = self.templates.get(template_name).unwrap();
        delete_template_by_path(template_path)?;
        println!(
            "{} Done in {}",
            OPERATION_HAS_BEEN_COMPLETED_EMOJI,
            HumanDuration(started.elapsed())
        );
        Ok(())
    }
}
