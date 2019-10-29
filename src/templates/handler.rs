use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

use indicatif::HumanDuration;

use crate::constants::OPERATION_HAS_BEEN_COMPLETED_EMOJI;
use crate::error::Error;
use crate::filesystem::create_directory;
use crate::templates::config::Config;
use crate::templates::task::Task;

pub struct Handler;

impl Handler {
    pub fn new() -> Self where Self: Sized {
        Handler {}
    }

    /// Initializes the target directory with the specified templates.
    pub fn init_project(
        &self,
        target_directory_path: &String,
        templates: &HashMap<String, String>,
        configs: &HashMap<String, Box<Config>>
    ) -> Result<(), Error> {
        let started = Instant::now();
        let project_directory_path = PathBuf::from(target_directory_path);
        create_directory(&project_directory_path)?;

        for (template_name, config) in configs {
            let template_directory_path = PathBuf::from(templates.get(template_name).unwrap());
            let task = Task::new(&project_directory_path, &template_directory_path, config);
            task.run()?;
        }

        println!(
            "{} Done in {}",
            OPERATION_HAS_BEEN_COMPLETED_EMOJI,
            HumanDuration(started.elapsed())
        );
        Ok(())
    }
}
