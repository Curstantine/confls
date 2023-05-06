use clap::Parser;
use environment::structs::Environment;

mod aur;
mod cli;
mod data;
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
    let related_user_files = env.read().await?.to_related(&env)?;
    println!("{:#?}", related_user_files);

    // let package_install = aur::install_packages(
    //     env.setup.info.requires.clone(),
    //     env.shared.clone().map(|s| s.config.info.requires),
    // );

    // let data_conf = data::Data::new().await?;
    // data_conf.backup_env(&env).await?;

    // package_install.await?;

    Ok(())
}
