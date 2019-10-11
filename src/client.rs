use std::collections::HashMap;

use crate::cli::{Command, EntityTypeEnum, InstallerTypeEnum};
use crate::error::Error;
use crate::filesystem::{
    create_directory, delete_repository, get_templates_directory,
    get_repositories_map, get_templates_map, is_template_exist
};
use crate::installers::git::GitInstaller;
use crate::installers::traits::TemplateInstaller;
use crate::managers::{Manager, RepositoryManager, TemplateManager};
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
            create_directory(&templates_directory)?;
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
            Command::Init {
                plugins,
                options
            } => self.init_project(plugins, options),
            Command::Install {
                installer_type,
                path,
                template_name,
            } => self.install_template(installer_type, path, template_name),
            Command::List {
                entity
            } => self.show_entity_list(entity),
            Command::Delete {
                entity,
                name
            } => self.delete_entity_by_name(entity, name),
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

    fn show_entity_list(&self, entity: &EntityTypeEnum) -> Result<(), Error> {
        let manager: Box<dyn Manager> = match entity {
            EntityTypeEnum::Repository => {
                let manager = RepositoryManager::new(&self.repositories);
                Box::new(manager)
            },
            EntityTypeEnum::Template => {
                let manager = TemplateManager::new(&self.templates);
                Box::new(manager)
            },
        };

        manager.show_entity_list()
    }

    fn delete_entity_by_name(&self, entity: &EntityTypeEnum, name: &String) -> Result<(), Error> {
        let manager: Box<dyn Manager>= match entity {
            EntityTypeEnum::Repository => {
                let manager = RepositoryManager::new(&self.repositories);
                Box::new(manager)
            },
            EntityTypeEnum::Template => {
                let manager = TemplateManager::new(&self.templates);
                Box::new(manager)
            },
        };

        manager.delete_entity(name)
    }
}
