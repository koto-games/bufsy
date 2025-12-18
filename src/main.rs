use anyhow::Result;
use clap::Parser;
use tokio;

mod app;
mod cli;
mod transport;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    cli.run().await?;

    Ok(())
}
