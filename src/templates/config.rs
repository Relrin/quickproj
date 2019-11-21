use std::collections::HashMap;
use std::default::Default;
use std::fs::File;
use std::io::prelude::Read;
use std::path::PathBuf;

use regex::Regex;
use serde::Deserialize;
use serde_json::{json, Map as SerdeMap, Value as SerdeValue};
use lazy_static::lazy_static;

use crate::error::Error;
use crate::filesystem::CONFIG_NAME;

lazy_static! {
    static ref DEFAULT_TARGET_DIRECTORY: String = String::from(".");
    static ref REFERENCE_VARIABLE_REGEX: Regex = Regex::new(r"\{\{\s*\bVars.(?P<name>[\w\d_-]*)\b\s*}}").unwrap();
}

#[derive(Debug, Clone)]
pub struct Config {
    pub project_name: Option<String>,
    pub template_name: Option<String>,
    pub json_config: JsonConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JsonConfig {
    pub files: FilesConfig,
    pub variables: Option<HashMap<String, SerdeValue>>,
    pub scripts: Option<ScriptsConfig>,
    pub storage: Option<StorageConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FilesConfig {
    pub sources: Vec<HashMap<String, String>>,
    pub generated: Option<Vec<String>>,
    pub directories: Option<Vec<String>>,
    pub templates: Option<HashMap<String, String>>
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScriptsConfig {
    pub after_init: Option<Vec<String>>
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    pub variables: Option<HashMap<String, SerdeValue>>,
}

impl Config {
    pub fn new(json_config: JsonConfig) -> Self {
        Config {
            project_name: None,
            template_name: None,
            json_config
        }
    }

    pub fn with_project_name(mut self, project_name: &String) -> Self {
        self.project_name = Some(project_name.to_owned());
        self
    }

    pub fn with_template_name(mut self, template_name: &String) -> Self {
        self.template_name = Some(template_name.to_owned());
        self
    }

    pub fn get_source_entries(&self, template_path: &PathBuf) -> Vec<HashMap<String, String>> {
        self.json_config.files.sources.clone()
            .iter()
            .map(|entry| {
                let from_path = entry.get("from").unwrap();
                let updated_from_path = match from_path.starts_with(".") {
                    true => template_path.clone().to_str().unwrap().to_string(),
                    false => {
                        template_path
                            .clone()
                            .join(PathBuf::from(from_path))
                            .into_os_string()
                            .into_string()
                            .unwrap()
                    }
                };

                let to_path = entry
                    .get("to")
                    .unwrap_or(&DEFAULT_TARGET_DIRECTORY);
                let updated_to_path = match to_path.starts_with("./") {
                    true => to_path.replacen("./", "", 1),
                    false => to_path.clone(),
                };

                let mut updated_entry = HashMap::new();
                updated_entry.insert("from".to_string(), updated_from_path.to_string());
                updated_entry.insert("to".to_string(), updated_to_path.to_string());

                entry.clone()
                    .into_iter()
                    .chain(updated_entry.clone())
                    .collect()
            })
            .collect()
    }

    pub fn get_template_context(&self) -> Box<SerdeValue> {
        let mut context = SerdeMap::new();
        self.add_config_definition_to_context(&mut context);
        self.add_variables_to_context(&mut context);
        Box::new(json!(context))
    }

    fn add_config_definition_to_context(&self, context: &mut SerdeMap<String, SerdeValue>) {
        context.insert(
            "project_name".to_string(),
            SerdeValue::String(self.project_name.clone().unwrap_or("unknown".to_string()))
        );
        context.insert(
            "template_name".to_string(),
            SerdeValue::String(self.template_name.clone().unwrap_or("unknown".to_string()))
        );
    }

    fn add_variables_to_context(&self, context: &mut SerdeMap<String, SerdeValue>) {
        self.json_config.variables.clone().unwrap_or_default()
            .iter()
            .map(|(key, value)| self.inject_value_in_context(key, value, context))
            .collect()
    }

    fn inject_value_in_context(
        &self,
        key: &String,
        value: &SerdeValue,
        context: &mut SerdeMap<String, SerdeValue>
    ) {
        match value {
            SerdeValue::String(_) => {
                context.insert(key.to_string(), value.clone());
                ()
            },
            SerdeValue::Array(_) => {
                context.insert(key.to_string(), value.clone());
                ()
            },
            _ => {}
        }
    }
}

impl JsonConfig {
    pub fn from_file(path: &String) -> Result<JsonConfig, Error> {
        let mut data = String::new();
        File::open(path)
            .and_then(|mut f| f.read_to_string(&mut data))?;

        let mut json_config: JsonConfig = serde_json::from_str(&data)?;
        json_config.init_missing_fields();
        json_config.validate(path)?;
        Ok(json_config)
    }

    pub fn init_missing_fields(&mut self) {
        self.files.generated = Some(self.files.generated.clone().unwrap_or_default());
        self.files.directories = Some(self.files.directories.clone().unwrap_or_default());
        self.files.templates = Some(self.files.templates.clone().unwrap_or_default());
        self.variables = Some(self.variables.clone().unwrap_or_default());

        if self.scripts.is_none() {
            self.scripts = Some(ScriptsConfig::default());
        }

        if self.storage.is_none() {
            self.storage = Some(StorageConfig::default())
        }
    }

    pub fn validate(&self, config_path: &String) -> Result<(), Error> {
        for record in self.files.sources.iter() {
            if !record.contains_key("from") || !record.contains_key("to") {
                let message = format!(
                    "{} -> Each record in sources must have specified `from` and `to` \
                    keys. Please, check for correctness the config.json file.",
                    config_path.to_owned()
                );
                return Err(Error::Other(message))
            }
        }

        let overridable_variables = self.variables.clone().unwrap_or_default();
        self.validate_hashmap_values(config_path, "variables", &overridable_variables)?;

        let storage_variables = self.storage.clone().unwrap_or_default().variables.unwrap_or_default();
        self.validate_hashmap_values(config_path, "storage.variables", &storage_variables)?;
        self.validate_variable_references(config_path, &storage_variables)?;

        Ok(())
    }

    fn validate_hashmap_values(
        &self,
        config_path: &String,
        key_prefix: &str,
        hashmap: &HashMap<String, SerdeValue>,
    ) -> Result<(), Error> {
        for (key, serde_value) in hashmap {
            match serde_value {
                SerdeValue::String(_) => {},
                SerdeValue::Array(_) => {},
                _ => {
                    let message = format!(
                        "{}: The {}.{} key has unsupported value type. The configuration \
                        supports only string and array of strings types.",
                        config_path.to_owned(), key_prefix.to_owned(), key.to_owned()
                    );
                    return Err(Error::Other(message))
                }
            }
        };

        Ok(())
    }

    fn validate_variable_references(
        &self,
        config_path: &String,
        hashmap: &HashMap<String, SerdeValue>,
    ) -> Result<(), Error> {
        for (key, serde_value) in hashmap {
            match serde_value {
                SerdeValue::String(value) => {
                    self.validate_reference(config_path, key, value)?
                },
                SerdeValue::Array(array) => {
                    for item in array {
                        let value = format!("{}", item);
                        self.validate_reference(config_path, key, &value)?
                    }
                },
                _ => unreachable!(),
            };
        };

        Ok(())
    }

    fn validate_reference(
        &self,
        config_path: &String,
        key: &String,
        value: &String,
    ) -> Result<(), Error> {
        let escaped_value = value.trim_matches('\"');
        let is_template_variable =  escaped_value.starts_with("{{") && escaped_value.ends_with("}}");
        let is_invalid_reference = !REFERENCE_VARIABLE_REGEX.is_match(value);

        if is_template_variable && is_invalid_reference {
            let message = format!(
                "{}: The storage.variables.{} key has an invalid reference. \
                The invalid reference is `{}`. Please, make sure that the path \
                starts with `Vars.` and the specified variable exists.",
                config_path.to_owned(), key.to_owned(), value.to_owned()
            );
            return Err(Error::Other(message))
        }
        Ok(())
    }
}

impl Default for ScriptsConfig {
    fn default() -> Self {
        ScriptsConfig {
            after_init: Some(Vec::new())
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        StorageConfig {
            variables: Some(HashMap::new())
        }
    }
}

pub fn get_template_configs(
    project_name: &String,
    templates: &Vec<String>,
    defined_templates: &HashMap<String, String>,
) -> Result<HashMap<String, Box<Config>>, Error> {
    let mut configs = HashMap::new();

    for template_name in templates {
        let dir = defined_templates.get(template_name).unwrap();
        let path = format!("{}/{}", dir, CONFIG_NAME);
        let json_config = JsonConfig::from_file(&path)?;
        let config = Config::new(json_config)
            .with_project_name(&project_name.to_owned())
            .with_template_name(&template_name.to_owned());
        configs.insert(template_name.clone(), Box::new(config));
    }

    Ok(configs)
}
