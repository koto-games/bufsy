use anyhow::Result;
use clap::Parser;
use std::{fs, sync::LazyLock};

mod app;
mod cli;
mod transport;

static CONFIG_DIR: LazyLock<String> = LazyLock::new(|| {
    format!("{}/bufsy/", dirs::config_dir().unwrap().to_str().unwrap()).to_string()
});

static KEY: LazyLock<String> =
    LazyLock::new(|| fs::read_to_string(format!("{}/key", &*CONFIG_DIR)).unwrap());

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    cli.run().await?;

    Ok(())
}
