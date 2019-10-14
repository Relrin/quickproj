pub trait Handler {
    fn new() -> Self where Self: Sized;
    fn run_in_thread(&self, options: &Vec<String>);
}
