use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Other(message: String) {
            description(message)
            display("{}", message)
        }
    }
}
