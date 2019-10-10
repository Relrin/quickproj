use std::io::{stdin, stdout, Write};

use read_input::prelude::input;

use crate::error::Error;

static REPLACE_TEMPLATE_MESSAGE: &str =
    "The requested template already saved \
     in the application repository. \n\nYou can proceed the operation but it will \
     replace the existing template. Alternatively you could cancel the operation \
     and provide the `--template-name` option for the command.\nProceed? (y/n)";

pub fn ask_for_replacing_template() -> Result<(), Error> {
    println!("{}", REPLACE_TEMPLATE_MESSAGE);

    let input: String = input().get();
    match input.trim().to_lowercase().as_str() {
        "y" => Ok(()),
        _ => {
            let message = String::from("The operation was cancelled.");
            Err(Error::Other(message))
        }
    }
}
