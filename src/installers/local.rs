use std::time::Instant;

use bytesize::ByteSize;
use fs_extra::dir::{CopyOptions, TransitProcess, copy_with_progress};
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use quick_error::ResultExt;

use crate::constants::{COPYING_REPOSITORY_EMOJI, OPERATION_HAS_BEEN_COMPLETED_EMOJI};
use crate::error::Error;
use crate::filesystem::{basename, create_directory, get_templates_directory};
use crate::installers::traits::Installer;

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
        if state.copied_bytes != state.total_bytes {
            let copied_bytes = ByteSize::kb(state.copied_bytes);
            let total_bytes = ByteSize::kb(state.total_bytes);
            pb.set_length(state.total_bytes);
            pb.set_position(state.copied_bytes);
            pb.set_message(&format!(
                "[{}/{}] Copying...",
                copied_bytes, total_bytes
            ));
        } else {
            pb.set_style(ProgressStyle::default_bar().template("{wide_msg}"));
            pb.set_message(&format!(
                "{} The repository has been copied successfully...",
                COPYING_REPOSITORY_EMOJI
            ));
            pb.finish();
        }
    }
}

impl Installer for LocalInstaller {
    fn new() -> Self where Self: Sized {
        LocalInstaller {}
    }

    fn get_template_name(&self, path: &String) -> Result<String, Error> {
        let mut template_name = path.clone();
        if template_name.ends_with('/') {
            template_name.pop();
        }

        Ok(basename(&template_name, '/'))
    }

    fn install(&self, source: &String, template_name: &String) -> Result<(), Error> {
        let templates_folder = get_templates_directory()?;
        let destination = templates_folder.join(template_name);
        let started = Instant::now();

        let copy_pb = self.get_copy_progress_bar();
        create_directory(&destination);
        let options = CopyOptions::new();
        let handle = |state: TransitProcess|  {
            self.refresh_copy_progress_bar(&copy_pb, &state);
            fs_extra::dir::TransitProcessResult::OverwriteAll
        };
        copy_with_progress(&source, &destination, &options, handle)?;

        println!(
            "{} Done in {}",
            OPERATION_HAS_BEEN_COMPLETED_EMOJI,
            HumanDuration(started.elapsed())
        );
        Ok(())
    }
}