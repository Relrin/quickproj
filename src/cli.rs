use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "quickproj")]
pub struct CliOptions {
    #[structopt(name = "plugins")]
    pub plugins: Vec<String>,
}