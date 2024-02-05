pub mod signals;
pub mod config;
pub mod cache;
pub mod utils;

use salvo::conn::rustls::{
    Keycert,
    RustlsConfig
};
use salvo::{
    prelude::*,
    Listener,
    logging::Logger,
};
use config::Config;


/// Loads TLS configuration.
fn load_tls_config(
    config: &Config
) -> RustlsConfig
{
    RustlsConfig::new(
        Keycert::new()
            .cert(
                std::fs::read_to_string(config.ssl_certificate())
                    .unwrap()
            )
            .key(
                std::fs::read_to_string(config.ssl_key())
                    .unwrap()
            )
    )
}

fn load_proxy_config(
    config: &Config
) -> Service
{
    Service::new(
        Router::new()
            .path("<**rest>")
            .hoop()
            .goal(Proxy::default_hyper_client(config.upstream().to_string()))
    ).hoop(Logger::new())
}

/// Start a HTTP server.
async fn start_http_server(
    service: Service
)
{
    let acceptor =
        TcpListener::new("0.0.0.0:80")
            .bind()
            .await;

    let server = Server::new(acceptor);
    let handle = server.handle();

    tokio::spawn(signals::listen_shutdown_signal(handle));

    server.serve(service).await
}

/// Start a HTTPS server.
async fn start_https_server(
    service: Service,
    config : &Config
)
{
    let acceptor =
        TcpListener::new("0.0.0.0:443")
            .rustls(load_tls_config(config))
            .bind()
            .await;

    let server = Server::new(acceptor);
    let handle = server.handle();

    tokio::spawn(signals::listen_shutdown_signal(handle));

    server.serve(service).await
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let config = Config::load_config();

    let router = load_proxy_config(&config);

    if config.tls() {
        start_https_server(router, &config).await;
    } else {
        start_http_server(router).await;
    }
}
