use std::io::Error as StdIoError;
use git2::Error as Git2Error;
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: StdIoError) {
            from()
            description("io error")
            display("I/O error: {}", err)
            cause(err)
        }
        GitError(err: Git2Error) {
            from()
            description("git error")
            display("Git error: {}", err)
        }
        Other(message: String) {
            description(message)
            display("{}", message)
        }
    }
}
