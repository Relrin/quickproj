use std::cell::RefCell;
use std::time::Instant;

use git2::build::RepoBuilder;
use git2::{Config, FetchOptions, Progress, RemoteCallbacks};
use git2_credentials::CredentialHandler;
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};

use crate::constants::{CLONING_REPOSITORY_EMOJI, OPERATION_HAS_BEEN_COMPLETED_EMOJI};
use crate::error::Error;
use crate::filesystem::{basename, get_templates_directory};
use crate::installers::traits::TemplateInstaller;

struct State {
    progress: Option<Progress<'static>>,
}

pub struct GitInstaller;

impl GitInstaller {
    fn get_credential_helper(&self) -> Result<Box<CredentialHandler>, Error> {
        let git_config = Config::open_default()?;
        Ok(Box::new(CredentialHandler::new(git_config)))
    }

    fn get_download_progress_bar(&self) -> Box<ProgressBar> {
        let style = ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.green}] {msg}")
            .progress_chars("#>-");

        let download_pb = Box::new(ProgressBar::new(0));
        download_pb.set_style(style);
        download_pb
    }

    fn refresh_download_progress_bar(&self, pb: &ProgressBar, state: &mut State) {
        let stats = state.progress.as_ref().unwrap();
        let total_objects = stats.total_objects();
        let downloaded_objects = stats.received_objects();
        if downloaded_objects != total_objects {
            pb.set_length(total_objects as u64);
            pb.set_position(downloaded_objects as u64);
            pb.set_message(&format!(
                "[{}/{} objects] Cloning...",
                downloaded_objects, total_objects
            ));
        } else {
            pb.set_style(ProgressStyle::default_bar().template("{wide_msg}"));
            pb.set_message(&format!(
                "{} The repository has been cloned successfully...",
                CLONING_REPOSITORY_EMOJI
            ));
            pb.finish();
        }
    }
}

impl TemplateInstaller for GitInstaller {
    fn new() -> Self {
        GitInstaller {}
    }

    fn get_template_name(&self, url: &String) -> Result<String, Error> {
        let mut repository_name = url.clone();
        if repository_name.ends_with('/') {
            repository_name.pop();
        }

        if repository_name.ends_with(".git") {
            for _ in 0..4 {
                repository_name.pop();
            }
        }

        Ok(basename(&repository_name.as_str(), '/'))
    }

    fn install(&self, url: &String, template_name: &String) -> Result<(), Error> {
        let templates_folder = get_templates_directory()?;
        let destination = templates_folder.join(template_name);
        let started = Instant::now();
        let state = RefCell::new(State { progress: None });
        let download_pb = self.get_download_progress_bar();

        let mut ch = self.get_credential_helper()?;
        let mut cb = RemoteCallbacks::new();
        cb.credentials(move |url, username, allowed| {
            ch.try_next_credential(url, username, allowed)
        });
        cb.transfer_progress(|stats| {
            let mut state = state.borrow_mut();
            state.progress = Some(stats.to_owned());
            self.refresh_download_progress_bar(&download_pb, &mut *state);
            true
        });

        let mut fo = FetchOptions::new();
        fo.remote_callbacks(cb)
            .download_tags(git2::AutotagOption::All)
            .update_fetchhead(true);

        RepoBuilder::new()
            .branch("master")
            .fetch_options(fo)
            .clone(url, destination.as_path())?;

        println!(
            "{} Done in {}",
            OPERATION_HAS_BEEN_COMPLETED_EMOJI,
            HumanDuration(started.elapsed())
        );
        Ok(())
    }
}
