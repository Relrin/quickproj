use std::collections::HashMap;

use crate::cli::{Command, EntityTypeEnum, InstallerTypeEnum};
use crate::error::Error;
use crate::filesystem::{
    basename, create_directory, delete_repository_by_name,
    delete_template_by_path, get_templates_directory,
    get_repositories_map, get_templates_map, sanitize_path

};
use crate::installers::{GitInstaller, LocalInstaller, Installer};
use crate::managers::{Manager, RepositoryManager, TemplateManager};
use crate::templates::{Handler, is_correct_template_list, get_template_configs};
use crate::terminal::ask_for_replacing_template;
use serde::private::ser::constrain;

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
                target,
                templates
            } => self.init_project(target, templates),
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

    // TODO: Implement excluding and capturing config variables for the usage
    fn init_project(
        &self,
        target_directory: &String,
        templates: &Vec<String>
    ) -> Result<(), Error> {
        is_correct_template_list(templates, &self.templates)?;
        let target = sanitize_path(target_directory);
        let project_name = basename(&target, '/');
        let configs = get_template_configs(&project_name, templates, &self.templates)?;
        let handler = Handler::new();
        handler.init_project(&target, &self.templates, &configs)
    }

    fn install_template(
        &self,
        installer_type: &InstallerTypeEnum,
        path: &String,
        template_name: &Option<String>,
    ) -> Result<(), Error> {
        let worker = self.get_installer_from_enum(installer_type);
        let used_template_name = template_name
            .clone()
            .unwrap_or(worker.get_template_name(path)?);

        let is_template_exist = self.templates.contains_key(&used_template_name);
        let is_repository_exist = self.repositories.contains_key(&used_template_name);
        if is_repository_exist || is_repository_exist {
            ask_for_replacing_template()?;

            match is_template_exist {
                true => {
                    let path = self.templates.get(&used_template_name).unwrap();
                    delete_template_by_path(path)?
                },
                false => delete_repository_by_name(&used_template_name)?
            };
        }

        worker.install(path, &used_template_name)
    }

    fn show_entity_list(&self, entity: &EntityTypeEnum) -> Result<(), Error> {
        let manager = self.get_manager_from_enum(entity);
        manager.show_entity_list()
    }

    fn delete_entity_by_name(&self, entity: &EntityTypeEnum, name: &String) -> Result<(), Error> {
        let manager = self.get_manager_from_enum(entity);
        manager.delete_entity(name)
    }

    fn get_installer_from_enum(&self, value: &InstallerTypeEnum) -> Box<dyn Installer> {
        match value {
            InstallerTypeEnum::Git => Box::new(GitInstaller::new()),
            InstallerTypeEnum::Local => Box::new(LocalInstaller::new()),
        }
    }

    fn get_manager_from_enum(&self, value: &EntityTypeEnum) -> Box<dyn Manager> {
        match value {
            EntityTypeEnum::Repository => Box::new(RepositoryManager::new(&self.repositories)),
            EntityTypeEnum::Template => Box::new(TemplateManager::new(&self.templates)),
        }
    }
}
