use std::collections::{HashMap, BTreeSet};
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::PathBuf;

use handlebars::Handlebars;
use quick_error::ResultExt;
use regex::Regex;
use serde_json::{json, Value as SerdeValue};
use lazy_static::lazy_static;

use crate::error::Error;

lazy_static! {
    pub static ref UNKNOWN_VARIABLE_REGEX: Regex = Regex::new(r"(?P<var>\{{2}(?P<name>.{1,}?)\}{2})").unwrap();
    pub static ref TEMPLATE_VARIABLE_REGEX: Regex = Regex::new(r"\{\{\s*\b(?P<name>[\w\d_-]*)\b\s*}}").unwrap();
}

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

/// Extract all template variables in according to the TEMPLATE_VARIABLE_REGEX regex.
pub fn get_template_variables(data: &String) -> BTreeSet<String> {
    let mut used_variables = BTreeSet::new();
    for capture in TEMPLATE_VARIABLE_REGEX.captures_iter(data) {
        let value: String = capture["name"].to_string();
        used_variables.insert(value);
    }
    used_variables
}

/// Generates new file based on the template with specified context
pub fn generate_file_from_template(
    handlebars: &Handlebars,
    context: &SerdeValue,
    template_path: &PathBuf,
    out_file_path: &PathBuf,
) -> Result<(), Error> {
    let mut template = read_to_string(template_path).context(template_path)?;

    // Because the Handlebars could throw panics for unknown variables, we need
    // to sanitize those things and then revert them after the rendering
    let defined_variables: Vec<String> = context
        .as_object().unwrap()
        .keys()
        .map(|key| key.to_owned())
        .collect();

    let missing_keys_substitutions = UNKNOWN_VARIABLE_REGEX.captures_iter(&template)
        .map(|captures| {
            let captured_block = captures["var"].to_string();
            let variable_name = captures["name"].trim().to_string();
            (captured_block, variable_name)
        })
        .filter(|(_captured_block, variable_name)| {
            !defined_variables.contains(variable_name)
        })
        .map(|(captured_block, _variable_name)| {
            let replacement = format!("{{{{{}}}}}", captured_block).to_string();
            (captured_block, replacement)
        })
        .collect::<Vec<(String, String)>>();

    missing_keys_substitutions
        .iter()
        .for_each(|(captured_block, replacement)| {
            template = template.replacen(captured_block, replacement, 1);
        });

    let mut module = handlebars
        .render_template(&template, &context)
        .context(out_file_path)?;

    // And reverse changes back
    missing_keys_substitutions
        .iter()
        .for_each(|(captured_block, replacement)| {
            module = module.replacen(replacement, captured_block, 1);
        });

    let mut file = File::create(out_file_path).context(out_file_path)?;
    file.write_all(module.as_bytes()).context(out_file_path)?;
    Ok(())
}

/// Merges two template contexts together with filtering by keys.
pub fn merge_contexts(
    a: &mut SerdeValue,
    b: &SerdeValue,
    keys: &BTreeSet<String>,
) {
    match (a, b) {
        (&mut SerdeValue::Object(ref mut a), &SerdeValue::Object(ref b)) => {
            for (key, value) in b {
                let string_key = key.to_string();
                if keys.contains(&string_key) && a.contains_key(&string_key) {
                    continue
                }

                merge_contexts(a.entry(key.clone()).or_insert(SerdeValue::Null), value, keys)
            }
        },
        (a, b) => {
            *a = b.clone();
        },
    }
}

/// Generate all possible combinations of subcontexts that will be used for the template.
pub fn generate_subcontexts(
    context: &Box<SerdeValue>,
    variables: &BTreeSet<String>
) -> Vec<SerdeValue> {
    let mut data = variables
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
