use crate::app::encryption::generate_key;
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Settings {
    pub server: Server,
    pub key: String,
    pub connections: Vec<Server>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

impl Settings {
    pub fn new_connection(&mut self, host: &str, port: u16) {
        self.connections.push(Server {
            host: host.to_string(),
            port,
        });
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            server: Server {
                host: String::from(local_ip().unwrap().to_string()),
                port: 9296,
            },
            key: hex::encode(generate_key()),
            connections: Vec::new(),
        }
    }
}
