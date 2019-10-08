use crate::cli::{InstallerTypeEnum, Command};
use crate::error::Error;
use crate::filesystem::{create_directory, get_template_directory};
use crate::plugins::{Plugin, is_correct_plugins_list};

pub struct Client;

impl Client {
    pub fn new() -> Self {
        Client {}
    }

    pub fn run(&self, command: &Command) {
        let result = match command {
            Command::Init {
                plugins,
                options,
            } => self.init_project(plugins, options),
            Command::Install {
                installer_type,
                path,
            } => self.install_template(installer_type, path),
            Command::List {
            } => self.show_template_list(),
        };

        match result {
            Ok(_) => {},
            Err(err) => println!("{}", err),
        }
    }

    fn init_project(&self, plugins: &Vec<String>, options: &Vec<String>) -> Result<(), Error> {
        Ok(())
    }

    fn install_template(&self, installer: &InstallerTypeEnum, path: &String) -> Result<(), Error> {
        let template_directory = get_template_directory()?;
        create_directory(&template_directory);
        Ok(())
    }

    fn show_template_list(&self) -> Result<(), Error> {
        Ok(())
    }
}