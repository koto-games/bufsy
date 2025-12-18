use super::commands::Commands;
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub async fn run(&self) -> Result<()> {
        self.command.run().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_name() {
        Cli::run(&Cli {
            command: Commands::Move {},
        })
        .await
        .unwrap();
    }
}
