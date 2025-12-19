use crate::app::encryption::generate_key;
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub server: Server,
    pub key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            server: Server {
                host: String::from(local_ip().unwrap().to_string()),
                port: 9296,
            },
            key: hex::encode(generate_key()),
        }
    }
}
