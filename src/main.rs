mod cli;
mod client;
mod constants;
mod error;
mod filesystem;
mod managers;
mod installers;
mod templates;
mod terminal;

use structopt::StructOpt;

use crate::cli::Command;
use crate::client::Client;

fn main() {
    let command = Command::from_args();
    match Client::new() {
        Ok(client) => client.run(&command),
        Err(err) => println!("{}", err),
    }
}
