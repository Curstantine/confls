use clap::Parser;

mod cli;
mod environment;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    let _files = environment::read_env(&args.name, &args.config_dir, args.no_root).await?;

    Ok(())
}
