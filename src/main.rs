use anyhow::Result;
use clap::Parser;

mod app;
mod cli;
mod config;
mod transport;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    let config_dir = format!("{}/bufsy", dirs::config_dir().unwrap().to_str().unwrap());

    cli.run(config_dir).await?;

    Ok(())
}
