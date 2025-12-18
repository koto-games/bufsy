use crate::{
    app::{
        encryption::{decrypt, encrypt, generate_key, generate_nonce},
        fnthost,
    },
    transport::{Backend, axum::ServerAXUM},
};
use anyhow::Result;
use clap::Subcommand;
use local_ip_address::local_ip;

#[derive(Subcommand)]
pub enum Commands {
    /// Run запуск
    Run {
        #[arg(default_value_t = String::from(local_ip().unwrap().to_string()))]
        ip: String,

        #[arg(default_value_t = 9296)]
        port: u16,
    },
    /// перемещение
    Move {},
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
                let message = "Привет, это секретное сообщение!";

                println!("Исходное сообщение: {}", message);
                println!("\n--- ШИФРОВАНИЕ ---");

                // Шифруем
                match encrypt(message) {
                    Ok((key_hex, nonce_hex, ciphertext_hex)) => {
                        println!("Ключ (hex):       {}", key_hex);
                        println!("Nonce (hex):      {}", nonce_hex);
                        println!("Зашифровано (hex): {}", ciphertext_hex);

                        println!("\n--- ДЕШИФРОВАНИЕ ---");

                        // Дешифруем
                        match decrypt(&key_hex, &nonce_hex, &ciphertext_hex) {
                            Ok(decrypted) => {
                                println!("Расшифровано: {}", decrypted);

                                if decrypted == message {
                                    println!("\n✓ Успешно! Сообщения совпадают.");
                                }
                            }
                            Err(e) => println!("Ошибка: {}", e),
                        }
                    }
                    Err(e) => println!("Ошибка: {}", e),
                }
            }
        }
        Ok(())
    }
}
