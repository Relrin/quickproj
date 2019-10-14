use clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug)]
    pub enum InstallerTypeEnum {
        Git,
        Local,
    }
}

arg_enum! {
    #[derive(Debug)]
    pub enum EntityTypeEnum {
        Repository,
        Template,
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
            help = "Additional options for plugins"
        )]
        options: Vec<String>,
    },
    /// Install new template into the default directory
    #[structopt(name = "install")]
    Install {
        #[structopt(
            raw(possible_values = "&InstallerTypeEnum::variants()"),
            name = "installer",
            help = "Used plugin for installing templates.",
            case_insensitive = true
        )]
        installer_type: InstallerTypeEnum,
        #[structopt(
            name = "path",
            help = "Path to the repository with the template."
        )]
        path: String,
        #[structopt(
            long = "--template-name",
            help = "Custom name for the installed template."
        )]
        template_name: Option<String>,
    },
    /// Show list of the available repositories or templates
    #[structopt(name = "list")]
    List {
        #[structopt(
            raw(possible_values = "&EntityTypeEnum::variants()"),
            name = "entity",
            help = "The name of the deleted type.",
            case_insensitive = true
        )]
        entity: EntityTypeEnum,
    },
    /// Delete one of the installed repositories or templates
    #[structopt(name = "delete")]
    Delete {
        #[structopt(
            raw(possible_values = "&EntityTypeEnum::variants()"),
            name = "entity",
            help = "The name of the deleted type.",
            case_insensitive = true
        )]
        entity: EntityTypeEnum,
        #[structopt(help = "Name of the installed repository or the template.")]
        name: String,
    },
}
