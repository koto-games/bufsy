use crate::{
    app::fnthost,
    cli::init,
    transport::{Backend, axum::ServerAXUM},
};
use anyhow::Result;
use clap::Subcommand;
use local_ip_address::local_ip;

#[derive(Subcommand)]
pub enum Commands {
    /// запуск
    Run {
        #[arg(default_value_t = String::from(local_ip().unwrap().to_string()))]
        ip: String,

        #[arg(default_value_t = 9296)]
        port: u16,
    },

    /// перемещение
    Move {},

    /// инициализация
    Init {},
}

impl Commands {
    pub async fn run(&self) -> Result<()> {
        match &self {
            Commands::Run { ip, port } => {
                println!("{}:{}", ip, *port);
                let mut ser = Backend::Axum(ServerAXUM::new(ip, *port, fnthost));
                ser.run().await?;
            }
            Commands::Move {} => {
                println!("move");
            }
            Commands::Init {} => {
                init(None).await?;
            }
        }
        Ok(())
    }
}
