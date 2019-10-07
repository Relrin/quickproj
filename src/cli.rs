use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "quickproj")]
pub enum Command {
    /// Initialize a new project with specified templates
    #[structopt(
        name = "init",
        raw(setting = "structopt::clap::AppSettings::TrailingVarArg")
    )]
    Init {
        #[structopt(
            name = "plugins",
            help = "Used plugins for a project generation"
        )]
        plugins: Vec<String>,
        #[structopt(
            last = true,
            help = "Additional options for plugins",
        )]
        options: Vec<String>,
    },
    /// Install new template into the default directory
    #[structopt(name = "install")]
    Install {
    },
    /// Show list of available templates
    #[structopt(name = "list")]
    List {
    }
}