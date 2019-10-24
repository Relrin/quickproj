use std::collections::{HashMap, BTreeSet};
use std::fs::{File, read_to_string};
use std::io::Write;

use handlebars::Handlebars;
use quick_error::ResultExt;
use serde_json::{json, Value as SerdeValue};

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

/// Generates new file based on the template with specified context
pub fn generate_file_from_template(
    context: Box<SerdeValue>,
    template_path: &String,
    out_file_path: &String,
) -> Result<(), Error> {
    let handlebars = Handlebars::new();
    let template = read_to_string(template_path).context(template_path)?;

    let module = handlebars
        .render_template(&template, &context)
        .context(out_file_path)?;

    let mut file = File::create(out_file_path).context(out_file_path)?;
    file.write_all(module.as_bytes()).context(out_file_path)?;
    Ok(())
}

/// Generate all possible combinations of subcontexts that will be used for the template
pub fn generate_subcontexts(
    context: &Box<SerdeValue>,
    variables: &BTreeSet<String>
) -> Vec<SerdeValue> {
    let mut data: Vec<HashMap<String, SerdeValue>> = variables
        .iter()
        .filter(|variable_name| {
            let value = match context.get(*variable_name) {
                Some(value) => value,
                None => return false,
            };

            match value {
                SerdeValue::String(_) => true,
                SerdeValue::Array(_) => true,
                _ => false,
            }
        })
        .map(|variable_name| {
            let mut entry: HashMap<String, SerdeValue> = HashMap::new();
            entry.insert(variable_name.clone(), context.get(variable_name).unwrap().clone());
            entry
        })
        .collect();

    get_combinations(&mut data)
        .iter()
        .map(|combination| json!(combination))
        .collect()
}

/// Generate all possible combinations for the given context without repeats.
///
/// Currently getting combinations is supported for hashmaps where values
/// represented as the String or as the Vec<String> types.
fn get_combinations(
    data: &mut Vec<HashMap<String, SerdeValue>>
) -> Vec<HashMap<String, String>> {
    if data.is_empty() {
        return Vec::new()
    }

    let mut combinations: Vec<HashMap<String, String>> = Vec::new();
    while !data.is_empty() {
        let variable_data = data.pop().unwrap();
        let mut pairs: Vec<HashMap<String, String>> = Vec::new();
        for (key, value) in variable_data {
            match value {
                SerdeValue::String(value) => {
                    let mut hashmap = HashMap::new();
                    hashmap.insert(key.clone(), value.clone());
                    pairs.push(hashmap);
                    ()
                },
                SerdeValue::Array(values) => {
                    for value in values {
                        let stringified_value = match value {
                            SerdeValue::String(val) => val.clone(),
                            _ => String::from("unsupported type")
                        };

                        let mut hashmap = HashMap::new();
                        hashmap.insert(key.clone(), stringified_value);
                        pairs.push(hashmap);
                    }
                    ()
                },
                _ => ()
            }
        }

        combinations = cartesian_hashmap_product(&combinations, &pairs);
    }

    combinations
}

/// Computes a cartesian product between two hashmaps.
fn cartesian_hashmap_product(
    current_pairs: &Vec<HashMap<String, String>>,
    pairs: &Vec<HashMap<String, String>>,
) -> Vec<HashMap<String, String>>{
    if current_pairs.is_empty() {
        return pairs.clone()
    }

    let mut cartesian_product = Vec::new();
    for current_pair in current_pairs {
        for addition in pairs {
            let new_pair = current_pair.clone()
                .into_iter()
                .chain(addition.clone())
                .collect();
            cartesian_product.push(new_pair)
        }
    }
    cartesian_product
}
