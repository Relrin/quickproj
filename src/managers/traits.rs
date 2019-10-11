use std::collections::HashMap;

use crate::error::Error;

pub trait Manager {
    fn new(entities: &HashMap<String, String>) -> Self where Self: Sized;
    fn show_entity_list(&self) -> Result<(), Error>;
    fn delete_entity(&self, name: &String) -> Result<(), Error>;
}