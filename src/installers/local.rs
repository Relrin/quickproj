use std::time::Instant;

use fs_extra::TransitProcess;
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};

use crate::constants::OPERATION_HAS_BEEN_COMPLETED_EMOJI;
use crate::error::Error;
use crate::filesystem::{basename, get_templates_directory};
use crate::installers::traits::TemplateInstaller;

pub struct LocalInstaller;

impl LocalInstaller {
    fn get_copy_progress_bar(&self) -> Box<ProgressBar> {
        let style = ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.green}] {msg}")
            .progress_chars("#>-");

        let download_pb = Box::new(ProgressBar::new(0));
        download_pb.set_style(style);
        download_pb
    }

    fn refresh_copy_progress_bar(&self, pb: &ProgressBar, state: &TransitProcess) {

    }
}

impl TemplateInstaller for LocalInstaller {
    fn new() -> Self where Self: Sized {
        LocalInstaller {}
    }

    fn get_template_name(&self, path: &String) -> Result<String, Error> {
        Ok(basename(path, '/'))
    }

    fn install(&self, path: &String, template_name: &String) -> Result<(), Error> {
        let templates_folder = get_templates_directory()?;
        let destination = templates_folder.join(template_name);

        let started = Instant::now();

        println!(
            "{} Done in {}",
            OPERATION_HAS_BEEN_COMPLETED_EMOJI,
            HumanDuration(started.elapsed())
        );
        Ok(())
    }
}