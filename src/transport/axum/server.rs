use anyhow::Result;
use axum::{
    Router,
    extract::State,
    {routing::get, routing::post},
};
use std::net::SocketAddr;

pub struct ServerAXUM {
    host: String,
    fnt: fn(String) -> Result<()>,
    port: u16,
}

impl ServerAXUM {
    pub fn new(host: &str, port: u16, fnt: fn(String) -> Result<()>) -> Self {
        Self {
            host: host.to_string(),
            fnt,
            port,
        }
    }

    fn router(&self) -> Router {
        Router::new()
            .route("/text", post(Self::text))
            .with_state(self.fnt)
            .route("/", get("Bufsy"))
    }

    // The `text` handler now takes the `fnt` function pointer directly as state
    // using the `axum::extract::State` extractor.
    // This allows the handler to call the configured function without needing access to the
    // full `ServerAXUM` instance, resolving the `Handler` trait not satisfied error.
    async fn text(State(fnt_handler): State<fn(String) -> Result<()>>, body: String) -> String {
        fnt_handler(body.clone()).unwrap();
        "oK!".to_string()
    }

    pub fn address(&self) -> SocketAddr {
        SocketAddr::new(
            self.host.parse().unwrap_or_else(|_| {
                // If parsing as an `IpAddr` fails, default to the IPv4 loopback address (127.0.0.1).
                // This resolves the panic for "localhost" and provides a fallback for other
                // hostnames that cannot be directly parsed as IP addresses.
                std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)
            }),
            self.port,
        )
    }

    pub async fn run(&mut self) -> Result<()> {
        let app = self.router();

        let listener = tokio::net::TcpListener::bind(self.address()).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_new() {
        let server = ServerAXUM::new("111.168.11.75", 8084, fnt_test);
        assert_eq!(
            server.address().to_string(),
            "111.168.11.75:8084".to_string()
        );

        let server = ServerAXUM::new("localhost", 999, fnt_test);
        assert_eq!(server.address().to_string(), "127.0.0.1:999".to_string());
    }

    #[tokio::test]
    async fn text_echo() {
        let server = ServerAXUM::new("localhost", 8080, fnt_test);
        let result = ServerAXUM::text(State(server.fnt), "catRUST_***_rust w :>".to_string()).await;

        assert_eq!(result, "oK!".to_string());
    }

    fn fnt_test(_text: String) -> Result<()> {
        Ok(())
    }
}
