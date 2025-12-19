use crate::{
    app::{
        encryption::{encrypt, generate_nonce},
        fnthost,
    },
    config::Settings,
    transport::{Backend, axum::ServerAXUM},
};
use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand, PartialEq)]
pub enum Commands {
    /// запуск
    Run {
        ip: Option<String>,
        port: Option<u16>,
    },

    /// перемещение
    Move {},

    /// перемещение
    Echo { text: String },

    /// инициализация
    Init {},
}

impl Commands {
    pub async fn run(&self, _config_dir: String, config: Settings) -> Result<()> {
        match &self {
            Commands::Run { ip, port } => {
                let ip: String = ip.clone().unwrap_or(config.server.host.to_string());
                let port = port.unwrap_or(config.server.port);
                println!("{}:{}", ip, port);
                let mut ser = Backend::Axum(ServerAXUM::new(&ip, port, fnthost, config));
                ser.run().await?;
            }
            Commands::Move {} => {
                println!("move");
            }
            Commands::Echo { text } => {
                let client = reqwest::Client::new();
                let nonce = hex::encode(generate_nonce());
                let test_encrypted = format!(
                    "{}|{}|{}",
                    encrypt(text, &config.key, &nonce).unwrap(),
                    nonce,
                    config.server.port
                );
                println!(
                    "echo {}  {}",
                    test_encrypted,
                    format!("http://{}:{}/text", config.server.host, config.server.port)
                );
                let _resp = client
                    .post(format!(
                        "http://{}:{}/text",
                        config.server.host, config.server.port
                    ))
                    .body(test_encrypted)
                    .send()
                    .await?;
            }
            _ => {}
        }
        Ok(())
    }
}
