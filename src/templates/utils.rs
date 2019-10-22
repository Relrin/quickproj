use std::collections::HashMap;
use std::fs::{File, read_to_string};
use std::io::Write;

use handlebars::Handlebars;
use quick_error::ResultExt;
use serde_json::json;

use crate::error::Error;
use crate::templates::config::Config;

/// Checks that the specified templates are available to use.
pub fn is_correct_template_list(
    templates: &Vec<String>,
    defined_templates: &HashMap<String, String>,
) -> Result<(), Error> {
    if templates.is_empty() {
        let message = String::from("Please, specify at least one used template and try again.");
        return Err(Error::Other(message));
    }

    let invalid_template_names: Vec<String> = templates
        .into_iter()
        .filter(|name| !defined_templates.contains_key(name.clone()))
        .map(|name| name.clone())
        .collect();

    match invalid_template_names.is_empty() {
        true => Ok(()),
        false => {
            let values = invalid_template_names.join(", ");
            let message = String::from(format!(
                "The templates with the following names weren't found or not available: {}",
                values
            ));
            Err(Error::Other(message))
        }
    }
}

/// Generates files
pub fn generate_file_from_template(
    config: Box<Config>,
    template_path: &String,
    out_file_path: &String,
) -> Result<(), Error> {
    let handlebars = Handlebars::new();
    let template = read_to_string(template_path).context(template_path)?;

    // TODO: Extract config variables
    let template_parameters = json!({ "test": "data" });
    let module = handlebars
        .render_template(&template, &template_parameters)
        .context(out_file_path)?;

    let mut file = File::create(out_file_path).context(out_file_path)?;
    file.write_all(module.as_bytes()).context(out_file_path)?;
    Ok(())
}
