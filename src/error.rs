use git2::Error as Git2Error;
use fs_extra::error::Error as FsExtraCallError;
use quick_error::quick_error;
use serde_json::error::Error as SerdeJsonError;

use std::io::Error as StdIoError;

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
            description("git2 error")
            display("Git2 error: {}", err)
        }
        FsExtraError(err: FsExtraCallError) {
            from()
            description("fs_extra error")
            display("FsExtra lib error: {}", err)
        }
        SerdeError(err: SerdeJsonError) {
            from()
            description("serde_json error")
            display("SerdeJson lib error: {}", err)
        }
        Other(message: String) {
            description(message)
            display("{}", message)
        }
    }
}
