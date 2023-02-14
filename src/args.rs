use clap::Parser;

#[derive(Debug, Parser)]
#[clap(about = "Mighty Hooks")]
pub struct Args {
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    #[clap(about = "Start serving the app")]
    Serve,
    #[clap(about = "Config management")]
    Config(ConfigArgs),
    #[clap(about = "Show Mighty Hooks version")]
    Version,
}

#[derive(Debug, Parser)]
pub struct ConfigArgs {
    #[clap(subcommand)]
    pub cmd: ConfigCommand,
}

#[derive(Debug, Parser)]
pub enum ConfigCommand {
    #[clap(about = "Show loaded config")]
    Show,
    #[clap(about = "Show where config is loaded from")]
    Find
}
