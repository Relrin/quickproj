use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::time::Instant;

use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use indicatif::HumanDuration;
use handlebars::Handlebars;
use serde_json::Value as SerdeValue;

use crate::constants::OPERATION_HAS_BEEN_COMPLETED_EMOJI;
use crate::error::Error;
use crate::filesystem::{create_directory, get_directory_objects};
use crate::templates::config::Config;
use crate::templates::utils::{
    TEMPLATE_VARIABLE_REGEX, generate_file_from_template, generate_subcontexts,
    get_template_variables, merge_contexts
};

pub struct Handler {
    handlebars: Box<Handlebars>
}

impl Handler {
    pub fn new() -> Self where Self: Sized {
        Handler {
            handlebars: Box::new(Handlebars::new())
        }
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
            self.run_task(&project_directory_path, &template_directory_path, config)?;
        }

        println!(
            "{} Done in {}",
            OPERATION_HAS_BEEN_COMPLETED_EMOJI,
            HumanDuration(started.elapsed())
        );
        Ok(())
    }

    /// Runs task for preparing a new project from the template.
    fn run_task(
        &self,
        project_directory_path: &PathBuf,
        template_directory_path: &PathBuf,
        config: &Box<Config>
    ) -> Result<(), Error> {
        let context = config.get_template_context();
        self.create_directories(project_directory_path, config, &context)?;
        self.create_target_directories(project_directory_path, template_directory_path, config)?;
        self.copy_files(project_directory_path, template_directory_path, config)?;
        self.create_files_from_templates(project_directory_path, template_directory_path, config, &context)?;
        Ok(())
    }

    /// Creates directories based on templates specified in the config[files][directories] space.
    fn create_directories(
        &self,
        target_path: &PathBuf,
        config: &Box<Config>,
        context: &Box<SerdeValue>
    ) -> Result<(), Error> {
        config.clone().json_config.files.directories.unwrap_or_default()
            .iter()
            .filter(|directory| TEMPLATE_VARIABLE_REGEX.is_match(directory))
            .for_each(|directory| {
                let path_variables = get_template_variables(&directory);
                generate_subcontexts(context, &path_variables)
                    .iter()
                    .for_each(|subcontext| {
                        let template_path = self.handlebars
                            .render_template(directory, &subcontext)
                            .unwrap();
                        let generated_path = PathBuf::from(template_path);
                        let subdirectory_path = target_path.join(generated_path);
                        create_directory(&subdirectory_path).unwrap();
                    })
            });
        Ok(())
    }

    /// Creates directories based on the records in the config[files][source] space.
    fn create_target_directories(
        &self,
        target_path: &PathBuf,
        template_directory_path: &PathBuf,
        config: &Box<Config>
    ) -> Result<(), Error> {
        config.get_source_entries(template_directory_path)
            .iter()
            .map(|entry| entry.get("to").unwrap())
            .filter(|str_path| **str_path != String::from("."))
            .map(|path| PathBuf::from(path))
            .for_each(|path| {
                let directory_path = target_path.join(path);
                create_directory(&directory_path).unwrap();
            });
        Ok(())
    }

    /// Copy files from config[files][sources] into the config[files][to] directory.
    fn copy_files(
        &self,
        target_path: &PathBuf,
        template_directory_path: &PathBuf,
        config: &Box<Config>
    ) -> Result<(), Error> {
        config.get_source_entries(template_directory_path)
            .iter()
            .map(|entry| (entry.get("from").unwrap(), entry.get("to").unwrap()))
            .map(|(from_path, to_path)| {
                let updated_to_path = match to_path == "." {
                    true => target_path.clone(),
                    false => target_path.join(to_path),
                };
                (PathBuf::from(from_path), updated_to_path)
            })
            .for_each(|(from_path, to_path)| {
                 let items_to_copy = get_directory_objects(&from_path);
                 let mut options = CopyOptions::new();
                 options.overwrite = true;
                 copy_items(&items_to_copy, to_path, &options).unwrap();
            });
        Ok(())
    }

    /// Creates files specified in config[files][generated] with the prepared context.
    fn create_files_from_templates(&self,
        target_path: &PathBuf,
        root_template_path: &PathBuf,
        config: &Box<Config>,
        context: &Box<SerdeValue>
    ) -> Result<(), Error> {
        let generated_files = config.clone().json_config.files.generated.unwrap_or_default();
        config.clone().json_config.files.templates.unwrap_or_default()
            .iter()
            .for_each(|(template_name, template_path)| {
                // Get all template variables from the passed template
                let full_template_path = root_template_path.join(PathBuf::from(template_path));
                let template_data = read_to_string(full_template_path.clone()).unwrap();
                let template_variables = get_template_variables(&template_data);

                // Generate possible all variants of paths with the certain subcontext
                let mut templates: HashMap<PathBuf, Box<SerdeValue>> = HashMap::new();
                generated_files.clone()
                    .iter()
                    .filter(|path| path.ends_with(template_name))
                    .for_each(|path| {
                        // Let's start from the check for dynamic paths (if was specified)
                        let path_variables = get_template_variables(&path);
                        match path_variables.is_empty() {
                            // Path is static. Shared context for everything
                            true => {
                                templates.insert(PathBuf::from(path), context.clone());
                            },
                            // Path is dynamic. Therefore each path has its own unique subcontext
                            false => {
                                generate_subcontexts(context, &path_variables)
                                    .iter()
                                    // Generate all unique paths
                                    .map(|subcontext| {
                                        let template_path = self.handlebars
                                            .render_template(path, &subcontext)
                                            .unwrap();
                                        let generated_path = PathBuf::from(template_path);
                                        (target_path.join(generated_path), subcontext)
                                    })
                                    // Then prepare a unique subcontext for each path
                                    .for_each(|(template_path, partial_context)| {
                                        let mut used_context = partial_context.clone();
                                        merge_contexts(&mut used_context, context, &template_variables);
                                        templates.insert(template_path.to_owned(), Box::new(used_context));
                                    });
                            }
                        };
                    });

                // And then generate all files with its own subcontext
                templates
                    .iter()
                    .for_each(|(target_file_path, subcontext)| {
                        generate_file_from_template(
                            &self.handlebars,
                            subcontext,
                            &full_template_path,
                            target_file_path
                        ).unwrap();
                    });
            });
        Ok(())
    }
}
