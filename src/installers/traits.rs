use crate::error::Error;

pub trait TemplateInstaller {
    fn new() -> Self where Self: Sized;
    fn get_template_name(&self, path: &String) -> Result<String, Error>;
    fn install(&self, path: &String, repository_name: &String) -> Result<(), Error>;
}
