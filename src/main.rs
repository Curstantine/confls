use clap::Parser;
use environment::structs::Environment;

mod aur;
mod cli;
mod environment;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli::Cli::parse();

    match args.command {
        cli::Commands::Set(set_args) => {
            set_workflow(&args.config_dir, set_args).await?;
        }
    }

    Ok(())
}

async fn set_workflow(config_dir: &str, args: cli::SetArgs) -> anyhow::Result<()> {
    let env = Environment::from_options(&args.name, config_dir, args.no_root).await?;
    let _files = env.read().await?;

    let package_install = aur::install_packages(
        env.setup.info.requires,
        env.shared.map(|s| s.config.info.requires),
    );

    package_install.await?;

    Ok(())
}
