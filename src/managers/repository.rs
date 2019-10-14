use std::collections::HashMap;
use std::time::Instant;

use indicatif::HumanDuration;

use crate::constants::OPERATION_HAS_BEEN_COMPLETED_EMOJI;
use crate::error::Error;
use crate::filesystem::delete_repository_by_name;
use crate::managers::traits::Manager;

pub struct RepositoryManager {
    repositories: HashMap<String, String>
}

impl Manager for RepositoryManager {
    fn new(repositories: &HashMap<String, String>) -> Self where Self: Sized {
        RepositoryManager {
            repositories: repositories.clone()
        }
    }

    fn show_entity_list(&self) -> Result<(), Error> {
        if self.repositories.is_empty() {
            println!("The templates folder is empty. Please, install templates first.");
            return Ok(())
        }

        let repositories = self.repositories
            .keys()
            .map(|key| &**key)
            .collect::<Vec<_>>()
            .join("\n  ");

        println!("Available repositories:\n  {}", repositories);
        Ok(())
    }

    fn delete_entity(&self, name: &String) -> Result<(), Error> {
        if self.repositories.is_empty() {
            println!("The templates folder is empty. Please, install templates first.");
            return Ok(())
        }

        if !self.repositories.contains_key(name) {
            println!("The name of the deleted repository is invalid. Please, make ensure \
                    that the template with this name exists.");
            return Ok(())
        }

        let started = Instant::now();
        delete_repository_by_name(name)?;
        println!(
            "{} Done in {}",
            OPERATION_HAS_BEEN_COMPLETED_EMOJI,
            HumanDuration(started.elapsed())
        );
        Ok(())
    }
}