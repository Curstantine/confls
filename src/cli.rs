use clap::{arg, command, Args, Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to the directory containing all the environment configurations.
    #[arg(long, default_value = "/code/configuration")]
    pub config_dir: String,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Set(SetArgs),
}

#[derive(Debug, Clone, Args)]
pub struct SetArgs {
    /// The name of the environment to set.
    pub name: String,

    /// Whether root actions should be performed.
    #[arg(long, default_value = "false")]
    pub no_root: bool,

    /// Whether to skip installing packages.
    #[arg(long, short, default_value = "false")]
    pub no_package_install: bool,
}
