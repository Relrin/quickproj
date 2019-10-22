use std::collections::HashMap;

use fs_extra::dir::copy;

use crate::error::Error;
use crate::templates::config::Config;
use crate::templates::utils::generate_file_from_template;
use serde_json::{json, Value, Map};

pub struct Handler;

impl Handler {
    pub fn new() -> Self where Self: Sized {
        Handler {}
    }

    // TODO: Replace on running in threads + add UI
    pub fn init_project(
        &self,
        target_directory_path: &String,
        templates: &HashMap<String, String>,
        configs: &HashMap<String, Box<Config>>
    ) -> Result<(), Error> {
        for (template_name, config) in configs {
            let template_directory_path = templates.get(template_name).unwrap();
            self.run_in_thread(target_directory_path, template_directory_path, config)?;
        }
        Ok(())
    }

    // 1. Create folders specified in config[files][to] and config[files][directories]
    // 2. Copy files from config[files][sources] into the config[files][to]
    // 3. Create file specified in config[files][templates] + provided the prepared context
    fn run_in_thread(
        &self,
        target: &String,
        template_directory_path: &String,
        config: &Box<Config>
    ) -> Result<(), Error> {
        let context = config.get_template_context();
        println!("{}", context);
        Ok(())
    }
}
