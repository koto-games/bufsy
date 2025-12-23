use crate::{
    app::{
        encryption::{encrypt, generate_nonce},
        fnthost,
    },
    cli::Cli,
    config::{Settings, load, save, settings::Server},
    transport::{Backend, axum::ServerAXUM},
};
use anyhow::Result;
use clap::{CommandFactory, Subcommand};
use std::io::{Read, Write};
use wl_clipboard_rs::paste::{ClipboardType, MimeType, Seat, get_contents};

#[derive(Subcommand, PartialEq)]
pub enum Commands {
    /// запуск
    Run {
        ip: Option<String>,
        port: Option<u16>,
    },

    /// Отправьте буфер
    Send {
        #[arg(short, long, value_name = "ADDRESS")]
        ip: Option<String>,
    },

    /// перемещение
    Echo {
        /// текст
        text: String,
        /// адрес
        #[arg(short, long, value_name = "ADDRESS")]
        ip: Option<String>,
    },

    /// инициализация
    Init {},

    /// ключ
    Key {
        #[command(subcommand)]
        command: KeyEnum,
    },

    /// генерация завершение
    GenerateCompletion { shells: clap_complete::Shell },
}

#[derive(Subcommand, PartialEq)]
pub enum KeyEnum {
    /// Показать ключ
    Show {},
    /// Установить ключ
    Set { key_update: String },
}

impl Commands {
    pub async fn run(&self, config_dir: &str, config: Settings) -> Result<()> {
        match &self {
            Commands::Run { ip, port } => {
                let ip: String = ip.clone().unwrap_or(config.server.host.to_string());
                let port = port.unwrap_or(config.server.port);
                println!("{}:{}", ip, port);
                let mut ser =
                    Backend::Axum(ServerAXUM::new(&ip, port, fnthost, config, config_dir));
                ser.run().await?;
            }
            Commands::Send { ip } => {
                let result =
                    get_contents(ClipboardType::Regular, Seat::Unspecified, MimeType::Text);
                match result {
                    Ok((mut pipe, _)) => {
                        let mut contents = vec![];
                        pipe.read_to_end(&mut contents)?;
                        let text = String::from_utf8_lossy(&contents).to_string();
                        println!("Pasted: {}", text);
                        send_message(&text, config.clone(), config_dir, ip.clone()).await?;
                    }

                    Err(wl_clipboard_rs::paste::Error::NoSeats)
                    | Err(wl_clipboard_rs::paste::Error::ClipboardEmpty)
                    | Err(wl_clipboard_rs::paste::Error::NoMimeType) => {
                        println!(
                            "The clipboard is empty or doesn't contain text, nothing to worry about."
                        );
                    }

                    Err(err) => Err(err)?,
                }
            }
            Commands::Echo { text, ip } => {
                println!("echo {}", text);
                send_message(text, config.clone(), config_dir, ip.clone()).await?;
            }
            Commands::Key { command } => match command {
                KeyEnum::Set { key_update } => {
                    let mut config_mut = config.clone();
                    config_mut.key = key_update.to_string();
                    println!("KEY \"{}\"", config_mut.key);
                    let mut input = String::new();
                    print!("Save key? (y/n): ");
                    std::io::stdout().flush().unwrap();

                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read input");

                    if input.trim().to_lowercase() == "cat" {
                        println!("cat!");
                    }

                    if input.trim().to_lowercase() == "y"
                        || input.trim().to_lowercase() == "yes"
                        || input.trim().to_lowercase() == "Д"
                        || input.trim().to_lowercase() == "да"
                    {
                        save(&config_mut, &config_dir).expect("Failed to save config");
                        println!("Key saved successfully!");
                    }
                }
                KeyEnum::Show {} => println!("KEY \"{}\"", config.key),
            },
            Commands::GenerateCompletion { shells } => {
                let mut cmd = Cli::command();
                clap_complete::generate(*shells, &mut cmd, "bufsy", &mut std::io::stdout());
            }
            _ => {}
        }
        Ok(())
    }
}

async fn send_message(
    message: &str,
    config: Settings,
    config_dir: &str,
    address: Option<String>,
) -> Result<()> {
    let client = reqwest::Client::new();

    let nonce = hex::encode(generate_nonce());
    let test_encrypted = format!(
        "{}|{}|{}",
        encrypt(&message, &config.key, &nonce).unwrap(),
        nonce,
        config.server.port
    );
    let mut resp: Option<Result<reqwest::Response, reqwest::Error>> = None;
    if let Some(address) = address.clone() {
        resp = Some(
            client
                .post(format!("http://{}/text", address))
                .body(test_encrypted.clone())
                .send()
                .await,
        );
    }
    for connection in config.connections {
        println!("http://{}:{}/text", connection.host, connection.port);
        let resp = client
            .post(format!(
                "http://{}:{}/text",
                connection.host, connection.port
            ))
            .body(test_encrypted.clone())
            .send()
            .await;
        if let Err(_e) = resp {
            println!(
                "Failed to send message to {}!",
                format!("{}:{}", connection.host, connection.port)
            );
        }
    }
    if let Some(resp) = resp {
        if let Err(e) = resp {
            println!("Failed to send message to server! '{}'", e);
        } else if let Ok(resp) = resp {
            if !resp.status().is_success() && resp.text().await.unwrap() != "oK" {
                return Ok(());
            }
            let address = address.clone().unwrap();
            let address_s: Vec<&str> = address.split(":").collect();
            if address_s.len() != 2 {
                return Err(anyhow::anyhow!("Invalid address format {}", address));
            }
            let mut config_mut = load(&config_dir);
            config_mut.new_connection(address_s[0], address_s[1].parse::<u16>()?);
            if load(&config_dir).connections.contains(&Server {
                host: address_s[0].to_string(),
                port: address_s[1].parse::<u16>()?,
            }) {
                return Ok(());
            }
            println!("Connection added! {} {:?}", config_dir, config_mut);
            save(&config_mut, &config_dir)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::config::load_config::tests::{test_config_dir, test_load_config};

    use super::*;

    #[tokio::test]
    async fn test_send_message() {
        let config = test_load_config();
        let config_dir = test_config_dir();

        send_message("text :>", config.clone(), &config_dir, None)
            .await
            .unwrap();
    }
}
