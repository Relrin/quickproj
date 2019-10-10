use crate::error::Error;

/// Checks that the specified plugins are available to use.
pub fn is_correct_plugins_list(
    plugins: &Vec<String>,
    valid_plugins: &Vec<String>,
) -> Result<(), Error> {
    if plugins.is_empty() {
        let message = String::from("Please, specify at least one used plugin and try again.");
        return Err(Error::Other(message));
    }

    let invalid_plugin_names: Vec<String> = plugins
        .into_iter()
        .filter(|plugin_name| !valid_plugins.contains(plugin_name))
        .map(|plugin_name| plugin_name.clone())
        .collect();

    match invalid_plugin_names.is_empty() {
        true => Ok(()),
        false => {
            let values = invalid_plugin_names.join(", ");
            let message = String::from(format!(
                "The following plugins weren't found or not available: {}",
                values
            ));
            Err(Error::Other(message))
        }
    }
}
