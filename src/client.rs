use crate::cli::CliOptions;
use crate::error::Error;

pub struct Client;

impl Client {
    pub fn new() -> Self {
        Client { }
    }

    pub fn run(&self, cli: &CliOptions) {
        match self.check_input(cli) {
            Ok(_) => self.prepare_project(cli),
            Err(err) => println!("{}", err),
        }
    }

    fn check_input(&self, cli: &CliOptions) -> Result<(), Error> {
        if cli.plugins.is_empty() {
            let message = String::from("Please, specify at least one used plugin and try again.");
            return Err(Error::Other(message))
        }

        Ok(())
    }

    fn prepare_project(&self, _cli: &CliOptions) {
    }
}