mod cli;
mod client;
mod error;
mod plugins;

use structopt::StructOpt;

use crate::cli::Command;
use crate::client::Client;

fn main() {
    let command = Command::from_args();
    let client = Client::new();
    client.run(&command);
}
