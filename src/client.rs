use std::collections::HashMap;
use std::path::Path;

use crate::cli::{Command, InstallerTypeEnum};
use crate::error::Error;
use crate::filesystem::{
    create_directory, delete_repository, get_templates_directory, get_templates_map,
    is_repository_exist,
};
use crate::installers::git::GitInstaller;
use crate::installers::traits::TemplateInstaller;
use crate::plugins::{is_correct_plugins_list, Plugin};
use crate::terminal::ask_for_replacing_template;

pub struct Client {
    templates: HashMap<String, String>,
}

impl Client {
    pub fn new() -> Result<Self, Error> {
        let templates_directory = get_templates_directory()?;
        if !templates_directory.exists() {
            create_directory(&templates_directory);
        }

        let templates_map = get_templates_map()?;
        Ok(Client { templates: templates_map })
    }

    pub fn run(&self, command: &Command) {
        let result = match command {
            Command::Init { plugins, options } => self.init_project(plugins, options),
            Command::Install {
                installer_type,
                path,
                template_name,
            } => self.install_template(installer_type, path, template_name),
            Command::List {} => self.show_template_list(),
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

        let repository_name = template_name
            .clone()
            .unwrap_or(worker.get_template_name(path)?);
        if is_repository_exist(&repository_name)? {
            ask_for_replacing_template()?;
            delete_repository(&repository_name)?;
        }

        worker.install(path, &repository_name)
    }

    fn show_template_list(&self) -> Result<(), Error> {
        Ok(())
    }
}
