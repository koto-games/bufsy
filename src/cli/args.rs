use super::commands::Commands;
use crate::{cli::init, config::load};
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub async fn run(&self, config_dir: String) -> Result<()> {
        if (Commands::Init {}) == self.command {
            init(&config_dir).await?;
            return Ok(());
        }
        let config = load(&config_dir);
        self.command.run(config_dir, config).await?;
        Ok(())
    }
}
