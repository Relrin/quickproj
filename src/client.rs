use crate::cli::Command;
use crate::error::Error;
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
            } => self.init_project(&plugins, &options),
            Command::Install {
            } => self.install_template(),
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

    fn install_template(&self) -> Result<(), Error> {
        Ok(())
    }

    fn show_template_list(&self) -> Result<(), Error> {
        Ok(())
    }
}