use clap::Parser;

use environment::structs::Environment;
use session::SessionConfig;

mod aur;
mod cli;
mod environment;
mod errors;
mod session;

#[tokio::main]
async fn main() -> errors::Result<()> {
    let args = cli::Cli::parse();

    match args.command {
        cli::Commands::Set(set_args) => {
            set_workflow(&args.config_dir, set_args).await?;
        }
    }

    Ok(())
}

async fn set_workflow(config_dir: &str, args: cli::SetArgs) -> errors::Result<()> {
    let env = Environment::from_options(&args.name, config_dir, args.no_root).await?;
    let related_user_files = env.read().await?.to_related(&env)?;

    if !args.no_package_install {
        aur::install_packages(
            env.setup.info.requires.clone(),
            env.shared.clone().map(|s| s.config.info.requires),
        )
        .await?;
    }

    let session_conf = SessionConfig::new().await?;
    session_conf.backup_env(&related_user_files).await?;

    Ok(())
}
