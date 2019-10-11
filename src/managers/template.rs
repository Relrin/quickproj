use std::collections::HashMap;
use std::time::Instant;

use indicatif::HumanDuration;

use crate::constants::OPERATION_HAS_BEEN_COMPLETED_EMOJI;
use crate::error::Error;
use crate::filesystem::delete_template_by_path;
use crate::managers::traits::Manager;

pub struct TemplateManager {
    templates: HashMap<String, String>
}

impl Manager for TemplateManager {
    fn new(templates: &HashMap<String, String>) -> Self where Self: Sized {
        TemplateManager {
            templates: templates.clone()
        }
    }

    fn show_entity_list(&self) -> Result<(), Error> {
        if self.templates.is_empty() {
            println!("The templates folder is empty. Please, install templates first.");
            return Ok(())
        }

        let templates = self.templates
            .keys()
            .map(|key| &**key)
            .collect::<Vec<_>>()
            .join("\n  ");

        println!("Available templates:\n  {}", templates);
        Ok(())
    }

    fn delete_entity(&self, template_name: &String) -> Result<(), Error> {
        if self.templates.is_empty() {
            println!("The templates folder is empty. Please, install templates first.");
            return Ok(())
        }

        if !self.templates.contains_key(template_name) {
            println!("The name of the deleted template is invalid. Please, make ensure \
                     that the template with this name exists.");
            return Ok(())
        }

        let started = Instant::now();
        let template_path = self.templates.get(template_name).unwrap();
        delete_template_by_path(template_path)?;
        println!(
            "{} Done in {}",
            OPERATION_HAS_BEEN_COMPLETED_EMOJI,
            HumanDuration(started.elapsed())
        );
        Ok(())
    }
}