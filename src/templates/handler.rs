use std::collections::{HashMap, BTreeSet};
use std::path::PathBuf;

use fs_extra::dir::copy;
use handlebars::Handlebars;
use regex::Regex;
use serde_json::{json, Value, Map};
use lazy_static::lazy_static;

use crate::error::Error;
use crate::filesystem::{basename, create_directory, sanitize_path};
use crate::templates::config::Config;
use crate::templates::utils::generate_file_from_template;

lazy_static! {
    static ref TEMPLATE_VARIABLE_REGEX: Regex = Regex::new(r"\{\{\s*\b(?P<name>[\w\d_-]*)\b\s*}}").unwrap();
}

pub struct Handler {
    handlebars: Box<Handlebars>
}

impl Handler {
    pub fn new() -> Self where Self: Sized {
        Handler {
            handlebars: Box::new(Handlebars::new())
        }
    }

    // TODO: Replace on running in threads + add UI
    pub fn init_project(
        &self,
        target_directory_path: &String,
        templates: &HashMap<String, String>,
        configs: &HashMap<String, Box<Config>>
    ) -> Result<(), Error> {
        let project_directory_path = PathBuf::from(target_directory_path);
        create_directory(&project_directory_path);

        for (template_name, config) in configs {
            let template_directory_path = templates.get(template_name).unwrap();
            self.run_in_thread(&project_directory_path, template_directory_path, config)?;
        }
        Ok(())
    }

    // 1. Create folders specified in config[files][to] and config[files][directories]
    // 2. Copy files from config[files][sources] into the config[files][to]
    // 3. Create file specified in config[files][templates] + provided the prepared context
    fn run_in_thread(
        &self,
        project_directory_path: &PathBuf,
        template_directory_path: &String,
        config: &Box<Config>
    ) -> Result<(), Error> {
        let context = config.get_template_context();
        self.create_directories(project_directory_path, config, &context);
        Ok(())
    }

    fn create_directories(
        &self,
        target_path: &PathBuf,
        config: &Box<Config>,
        context: &Box<Value>
    ) {
        config.clone().json_config.files.directories.unwrap_or_default()
            .iter()
            .filter(|directory| TEMPLATE_VARIABLE_REGEX.is_match(directory))
            .map(|directory| {
                let mut used_variables = BTreeSet::new();
                for capture in TEMPLATE_VARIABLE_REGEX.captures_iter(directory) {
                    let value: String = capture["name"].to_string();
                    used_variables.insert(value);
                }

                // generate all possible pairs for vec of ...
                // add variables with static data (string, i32, u64) into each pair
                // generate all kind of directory names and then create


//                let result = self.handlebars
//                    .render_template(directory, &context)
//                    .unwrap();
                  println!("{:?}", used_variables)
            })
            .collect()
    }
}
