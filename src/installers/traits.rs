use crate::error::Error;

pub trait Installer {
    fn new() -> Self where Self: Sized;
    fn get_template_name(&self, path: &String) -> Result<String, Error>;
    fn install(&self, path: &String, template_name: &String) -> Result<(), Error>;
}
