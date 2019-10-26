use std::collections::HashMap;
use std::default::Default;
use std::fs::File;
use std::io::prelude::Read;
use std::path::PathBuf;

use serde::Deserialize;
use serde_json::{json, Map as SerdeMap, Value as SerdeValue};
use lazy_static::lazy_static;

use crate::error::Error;
use crate::filesystem::CONFIG_NAME;

lazy_static! {
    static ref DEFAULT_TARGET_DIRECTORY: String = String::from(".");
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
    }

    pub fn validate(&self, config_path: &String) -> Result<(), Error> {
        if self.files.sources.is_empty() {
            let message = format!(
                "{} -> Sources can't be empty. Please, specify at least one record with \
                a directory or a file which have to be copied to the target directory.",
                config_path.to_owned()
            );
            return Err(Error::Other(message))
        }

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
