mod cli;
mod client;
mod error;

use structopt::StructOpt;

use crate::cli::CliOptions;
use crate::client::Client;

fn main() {
    let cli = CliOptions::from_args();
    let client = Client::new();
    client.run(&cli);
}
