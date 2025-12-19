use crate::config::Settings;
use anyhow::Result;
use axum::{
    Router,
    extract::{ConnectInfo, State},
    {routing::get, routing::post},
};
use std::net::SocketAddr;

pub struct ServerAXUM {
    host: String,
    fnt: fn(String, String, &Settings) -> Result<()>,
    port: u16,
    settings: Settings,
}

impl ServerAXUM {
    pub fn new(
        host: &str,
        port: u16,
        fnt: fn(String, String, &Settings) -> Result<()>,
        settings: Settings,
    ) -> Self {
        Self {
            host: host.to_string(),
            fnt,
            port,
            settings,
        }
    }

    fn router(&self) -> Router {
        // Use a single tuple state containing both the function pointer and the Settings.
        // This avoids requiring FromRef implementations for extracting multiple separate State<T>
        // values and keeps the handler signature simple.
        let app_state = (self.fnt, self.settings.clone());

        Router::new()
            .route("/text", post(Self::text))
            .route("/", get("Bufsy"))
            .with_state(app_state)
    }

    // The `text` handler now extracts the whole application state tuple `(fn, Settings)`
    // as a single `State` value and destructures it locally.
    async fn text(
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
        State((fnt_handler, settings)): State<(
            fn(String, String, &Settings) -> Result<()>,
            Settings,
        )>,
        body: String,
    ) -> String {
        // Call the configured function pointer with the received body and client IP.
        // Unwrap here mirrors the previous behavior in tests; consider proper error handling.
        fnt_handler(body.clone(), addr.ip().to_string(), &settings).unwrap();
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
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::load_config::tests::test_load_config;
    use std::net::{IpAddr, Ipv4Addr};

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn new_router() {
        let server = ServerAXUM::new("localhost", 8099, fnt_test, test_load_config());
        let app = server.router();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Bufsy");
    }

    #[test]
    fn address_new() {
        let server = ServerAXUM::new("111.168.11.75", 8084, fnt_test, test_load_config());
        assert_eq!(
            server.address().to_string(),
            "111.168.11.75:8084".to_string()
        );

        let server = ServerAXUM::new("localhost", 999, fnt_test, test_load_config());
        assert_eq!(server.address().to_string(), "127.0.0.1:999".to_string());
    }

    #[tokio::test]
    async fn text_echo() {
        let server = ServerAXUM::new("localhost", 8080, fnt_test, test_load_config());
        let result = ServerAXUM::text(
            ConnectInfo(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                8080,
            )),
            State((server.fnt, server.settings)),
            "catRUST_***_rust w :>".to_string(),
        )
        .await;

        assert_eq!(result, "oK!".to_string());
    }

    fn fnt_test(_text: String, _addr: String, _settings: &Settings) -> Result<()> {
        Ok(())
    }
}
