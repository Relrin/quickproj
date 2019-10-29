use read_input::InputBuild;
use read_input::prelude::input;
use serde_json::{json, Value as SerdeValue};

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

pub fn ask_for_input(
    template_name: &String,
    variable_name: &String,
    default_value: &SerdeValue
) -> Option<SerdeValue> {
    println!(
        "\nSpecify the value for the `{}:{}` key. Default is `{}`.",
        template_name, variable_name, default_value
    );

    let local_default = default_value.clone();
    let input: String = input()
        .repeat_msg("Input: ")
        .add_test(move |raw_value: &String| {
            match local_default {
                SerdeValue::Array(_) => {
                    let data: Vec<String> = raw_value
                        .split(",")
                        .collect::<Vec<&str>>()
                        .iter()
                        .filter(|raw_str| !raw_str.is_empty())
                        .map(|raw_str| String::from(*raw_str))
                        .collect();
                    !data.is_empty()
                },
                _ => true,
            }
        })
        .get();

    let user_data = input.trim().to_string();
    match user_data.is_empty() {
        false => match default_value {
            SerdeValue::String(_) => Some(SerdeValue::String(user_data)),
            SerdeValue::Array(_) => {
                let data: Vec<String> = user_data
                    .split(",")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|raw_str| String::from(*raw_str))
                    .collect();
                Some(json!(data))
            },
            _ => None
        },
        true => None,
    }
}
