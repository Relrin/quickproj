use clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug)]
    pub enum InstallerTypeEnum {
        Git,
        Local,
    }
}

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
            name = "options",
            help = "Additional options for plugins",
        )]
        options: Vec<String>,
    },
    /// Install new template into the default directory
    #[structopt(name = "install")]
    Install {
        #[structopt(
            raw(possible_values = "&InstallerTypeEnum::variants()"),
            name = "with",
            help = "Used plugin for installing templates.",
            case_insensitive = true
        )]
        installer_type: InstallerTypeEnum,
        #[structopt(
            name = "path",
            help = "Path to the repository with the template."
        )]
        path: String,
    },
    /// Show list of available templates
    #[structopt(name = "list")]
    List {
    }
}