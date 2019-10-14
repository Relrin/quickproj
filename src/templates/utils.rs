use std::collections::HashMap;

use crate::error::Error;

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
