use git2::Error as Git2Error;
use fs_extra::error::Error as FsExtraCallError;
use handlebars::TemplateRenderError;
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
        IoWithContext(err: StdIoError, source: String) {
            display("I/O error with {}: {}", source, err)
            context(source: &'a String, err: StdIoError)
                -> (err, source.to_string())
        }
        Git(err: Git2Error) {
            from()
            description("git2 error")
            display("Git2 error: {}", err)
        }
        FsExtra(err: FsExtraCallError) {
            from()
            description("fs_extra error")
            display("FsExtra lib error: {}", err)
        }
        Serde(err: SerdeJsonError) {
            from()
            description("serde_json error")
            display("SerdeJson lib error: {}", err)
        }
        TemplateRender(err: TemplateRenderError, filename: String) {
            display("Template rendering error for {} file: {}", filename, err)
            context(filename: &'a String, err: TemplateRenderError)
                -> (err, filename.to_string())
        }
        Other(message: String) {
            description(message)
            display("{}", message)
        }
    }
}
