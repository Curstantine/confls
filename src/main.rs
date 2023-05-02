use clap::Parser;

mod cli;
mod environment;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    let _files = utils::read_environment_files(&args.name, args.no_root).await?;

    Ok(())
}
