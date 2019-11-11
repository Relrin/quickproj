use std::collections::HashMap;
use std::env::set_current_dir;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Instant;

use indicatif::HumanDuration;
use quick_error::ResultExt;

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
        configs: &HashMap<String, Box<Config>>,
    ) -> Result<(), Error> {
        let started = Instant::now();
        let project_directory_path = PathBuf::from(target_directory_path);
        create_directory(&project_directory_path)?;

        for (template_name, config) in configs {
            let template_directory_path = PathBuf::from(templates.get(template_name).unwrap());
            let task = Task::new(&project_directory_path, &template_directory_path, config);
            task.run()?;
        }

        println!("Running post-hooks...");
        for (template_name, config) in configs {
            let scripts = config.json_config
                .scripts.clone().unwrap_or_default()
                .after_init.unwrap_or_default();

            for after_init_hook in scripts {
                let mut command: Vec<String> = after_init_hook
                    .split_ascii_whitespace()
                    .map(|slice| String::from(slice))
                    .collect();

                if command.is_empty() {
                    continue
                }

                let command_args = command.split_off(1);
                let mut process = Command::new(command[0].clone())
                    .args(&command_args)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .current_dir(project_directory_path.clone())
                    .spawn()
                    .context(&command[0])?;

                process.wait().context("spawn process")?;
            }
        }

        println!(
            "{} Done in {}",
            OPERATION_HAS_BEEN_COMPLETED_EMOJI,
            HumanDuration(started.elapsed())
        );
        Ok(())
    }
}
