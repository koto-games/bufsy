use anyhow::Result;

use super::axum::ServerAXUM;
// use super::udp::ServerUDP;

pub enum Backend {
    // Udp(ServerUDP),
    Axum(ServerAXUM),
}

impl Backend {
    pub async fn run(&mut self) -> Result<()> {
        match *self {
            // Backend::Udp(_) => Err(anyhow!("Missing attribute")),
            Backend::Axum(ref mut server) => server.run().await,
        }
    }
}
