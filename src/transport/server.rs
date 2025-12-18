use anyhow::{Result, anyhow};

use super::axum::ServerAXUM;
use super::udp::ServerUDP;

pub enum Backend {
    Udp(ServerUDP),
    Axum(ServerAXUM),
}

impl Backend {
    pub async fn run(&mut self) -> Result<()> {
        match *self {
            Backend::Udp(_) => Err(anyhow!("Missing attribute")),
            Backend::Axum(ref mut server) => server.run().await,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_name() {
//         let backend = Backend::Udp(server::run(AppState::default()).await.unwrap());
//         assert_eq!(backend, Backend::Udp(server::server));
//     }
// }
