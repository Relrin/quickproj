use std::collections::{HashMap, BTreeSet};
use std::fs::read_to_string;
use std::path::PathBuf;

use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use handlebars::Handlebars;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::Value as SerdeValue;

use crate::error::Error;
use crate::constants::{
    INITIALIZING_INSTALLATION_TASK_EMOJI, CREATING_DIRECTORIES_FROM_TEMPLATES_EMOJI,
    CREATING_DIRECTORIES_FOR_SOURCES_EMOJI, COPYING_FILES_TO_TARGET_EMOJI,
    GENERATING_FILES_FROM_TEMPLATES_EMOJI, INSTALLATION_TASK_HAS_FINISHED_EMOJI,
};
use crate::filesystem::{create_directory, get_directory_objects};
use crate::templates::config::Config;
use crate::templates::utils::{
    generate_file_from_template, generate_subcontexts, get_template_variables,
    merge_contexts
};

enum InstallStage {
    Started,
    CreatingTemplateDirectories,
    CreatingTargetDirectories,
    CopyingFiles,
    GeneratingFilesFromTemplates,
    Finished,
}

pub struct Task {
    handlebars: Box<Handlebars>,
    project_directory_path: PathBuf,
    template_directory_path: PathBuf,
    config: Box<Config>,
    progress_bar: Box<ProgressBar>,
}

impl Task {
    pub fn new(
        project_directory_path: &PathBuf,
        template_directory_path: &PathBuf,
        config: &Box<Config>
    ) -> Self {
        let style = ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.green}] {msg}")
            .progress_chars("#>-");
        let progress_bar = ProgressBar::new(0)
            .with_style(style);
        progress_bar.set_message(&format!("Initializing..."));
        progress_bar.set_length(5);

        Task {
            handlebars: Box::new(Handlebars::new()),
            project_directory_path: project_directory_path.to_owned(),
            template_directory_path: template_directory_path.to_owned(),
            config: config.to_owned(),
            progress_bar: Box::new(progress_bar),
        }
    }

    /// Runs task for preparing a new project from the template.
    pub fn run(&self) -> Result<(), Error> {
        self.refresh_progress_bar(InstallStage::Started);
        let context = self.config.get_template_context();
        self.create_template_directories(&context)?;
        self.create_target_directories()?;
        self.copy_files()?;
        self.create_files_from_templates(&context)?;
        self.refresh_progress_bar(InstallStage::Finished);
        Ok(())
    }

    /// Refreshing the progress bar that attached to the current task
    fn refresh_progress_bar(&self, state: InstallStage) {
        match state {
            InstallStage::Started => {
                self.progress_bar.set_message(&format!(
                    "{} Preparing contexts for the task...",
                    INITIALIZING_INSTALLATION_TASK_EMOJI,
                ));
            },
            InstallStage::CreatingTemplateDirectories => {
                self.progress_bar.inc(1);
                self.progress_bar.set_message(&format!(
                    "[1/4] {} Creating directories based on template definitions...",
                    CREATING_DIRECTORIES_FROM_TEMPLATES_EMOJI,
                ));
            },
            InstallStage::CreatingTargetDirectories => {
                self.progress_bar.inc(1);
                self.progress_bar.set_message(&format!(
                    "[2/4] {} Creating directories for the sources...",
                    CREATING_DIRECTORIES_FOR_SOURCES_EMOJI,
                ));
            },
            InstallStage::CopyingFiles => {
                self.progress_bar.inc(1);
                self.progress_bar.set_message(&format!(
                    "[3/4] {} Copying files into the target directory...",
                    COPYING_FILES_TO_TARGET_EMOJI,
                ));
            },
            InstallStage::GeneratingFilesFromTemplates => {
                self.progress_bar.inc(1);
                self.progress_bar.set_message(&format!(
                    "[4/4] {} Generating files from the templates...",
                    GENERATING_FILES_FROM_TEMPLATES_EMOJI,
                ));
            },
            InstallStage::Finished => {
                self.progress_bar.set_style(ProgressStyle::default_bar().template("{wide_msg}"));
                self.progress_bar.set_message(&format!(
                    "{} Installation of the `{}` template has been completed.",
                    INSTALLATION_TASK_HAS_FINISHED_EMOJI,
                    self.config.template_name.clone().unwrap(),
                ));
                self.progress_bar.finish();
            },
        };
    }

    /// Creates directories based on data specified in the config[files][directories] space.
    fn create_template_directories(&self, context: &Box<SerdeValue>) -> Result<(), Error> {
        self.refresh_progress_bar(InstallStage::CreatingTemplateDirectories);

        self.config.clone().json_config.files.directories.unwrap_or_default()
            .iter()
            .for_each(|directory| {
                let path_variables = get_template_variables(&directory);
                match path_variables.is_empty() {
                    // Path is static
                    true => {
                        let subdirectory_path = self.project_directory_path.join(directory);
                        create_directory(&subdirectory_path).unwrap();
                    },
                    // Path is dynamic. Therefore generate subcontexts and the create folders
                    false =>
                        generate_subcontexts(context, &path_variables)
                            .iter()
                            .for_each(|subcontext| {
                                let template_path = self.handlebars
                                    .render_template(directory, &subcontext)
                                    .unwrap();
                                let generated_path = PathBuf::from(template_path);
                                let subdirectory_path = self.project_directory_path.join(generated_path);
                                create_directory(&subdirectory_path).unwrap();
                            })
                }
            });
        Ok(())
    }

    /// Creates directories based on the records in the config[files][source] space.
    fn create_target_directories(&self) -> Result<(), Error> {
        self.refresh_progress_bar(InstallStage::CreatingTargetDirectories);

        self.config.get_source_entries(&self.template_directory_path)
            .iter()
            .map(|entry| entry.get("to").unwrap())
            .filter(|str_path| **str_path != String::from("."))
            .map(|path| PathBuf::from(path))
            .for_each(|path| {
                let directory_path = self.project_directory_path.join(path);
                create_directory(&directory_path).unwrap();
            });
        Ok(())
    }

    /// Copy files from config[files][sources] into the config[files][to] directory.
    fn copy_files(&self) -> Result<(), Error> {
        self.refresh_progress_bar(InstallStage::CopyingFiles);

        self.config.get_source_entries(&self.template_directory_path)
            .iter()
            .map(|entry| (entry.get("from").unwrap(), entry.get("to").unwrap()))
            .map(|(from_path, to_path)| {
                let updated_to_path = match to_path == "." {
                    true => self.project_directory_path.clone(),
                    false => self.project_directory_path.join(to_path),
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
    fn create_files_from_templates(&self, context: &Box<SerdeValue>) -> Result<(), Error> {
        self.refresh_progress_bar(InstallStage::GeneratingFilesFromTemplates);

        let generated_files = self.config.clone().json_config.files.generated.unwrap_or_default();
        self.config.clone().json_config.files.templates.unwrap_or_default()
            .iter()
            .for_each(|(template_name, template_path)| {
                // Get all template variables from the passed template
                let full_template_path = self.template_directory_path.join(PathBuf::from(template_path));
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
                                templates.insert(self.project_directory_path.join(path), context.clone());
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
                                        (self.project_directory_path.join(generated_path), subcontext)
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
