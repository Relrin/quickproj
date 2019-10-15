use crate::error::Error;

pub struct Handler;

impl Handler {
    fn new() -> Self where Self: Sized {
        Handler {}
    }

    fn run_in_thread(&self, options: &Vec<String>) -> Result<(), Error> {
        Ok(())
    }
}
