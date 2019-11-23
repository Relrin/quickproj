use std::collections::HashMap;
use std::io::Write;
use std::fs::{File, read_to_string};
use std::path::PathBuf;

use lazy_static::lazy_static;
use quick_error::ResultExt;
use regex::Regex;
use serde_json::Value as SerdeValue;

use crate::error::Error;

lazy_static! {
    pub static ref TEMPLATE_VARIABLE_REGEX: Regex = Regex::new(r"(?P<var>\{{2}(?P<name>.{1,}?)\}{2})").unwrap();
}


pub struct TemplateRenreder;

impl TemplateRenreder {
    pub fn new() -> Self {
        TemplateRenreder {}
    }

    /// Generates new file based on the template with specified context.
    pub fn generate_file_from_template(
        &self,
        context: &SerdeValue,
        template_path: &PathBuf,
        out_file_path: &PathBuf,
    ) -> Result<(), Error> {
        let template = read_to_string(template_path).context(template_path)?;
        let data = self.render_template(&template, context);
        let mut file = File::create(out_file_path).context(out_file_path)?;
        file.write_all(data.as_bytes()).context(out_file_path)?;
        Ok(())
    }

    /// Renders string from the given data and the context.
    pub fn render_template(&self, data: &String, context: &SerdeValue) -> String {
        let mut template = data.to_owned();
        let template_context = self.convert_context_into_hashmap(context);

        TEMPLATE_VARIABLE_REGEX.captures_iter(&template.clone())
            .map(|captures| {
                let captured_block = captures["var"].to_string().trim().to_string();
                let variable_name = captures["name"].trim().to_string();
                (captured_block, variable_name)
            })
            .filter(|(_captured_block, variable_name)| {
                template_context.contains_key(variable_name)
            })
            .for_each(|(captured_block, variable_name)| {
                let replacement = template_context.get(&variable_name).unwrap();
                template = template.replace(captured_block.as_str(), replacement);
            });

        template
    }

    /// Converts the given serde value object into plain hashmap.
    /// Any non-string key-value pairs will be ignored.
    fn convert_context_into_hashmap(&self, context: &SerdeValue) -> HashMap<String, String> {
        let mut template_context: HashMap<String, String> = HashMap::new();
        context
            .as_object().unwrap()
            .iter()
            .for_each(|(key, serde_value)| {
                match serde_value {
                    SerdeValue::String(value) => {
                        template_context.insert(key.to_owned(), value.to_owned());
                    },
                    _ => {},
                };
            });

        template_context
    }
}