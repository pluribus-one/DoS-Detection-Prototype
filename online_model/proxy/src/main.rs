pub mod signals;
pub mod config;
pub mod cache;
pub mod utils;
pub mod blocker;
pub mod metrics;


use salvo::{
    prelude::*,
    Listener,
    logging::Logger,
};
use config::Config;
use cache::IpsCache;
use blocker::Blocker;


fn load_proxy_config(
    config: &Config
) -> Service
{
    Service::new(
        Router::new()
            .path("<**rest>")
            .hoop(IpsCache::new(60))
            .hoop(Blocker)
            .goal(Proxy::default_hyper_client(config.upstream().to_string()))
    ).hoop(Logger::new())
}

// fn load_http_config(
//
// ) -> Service
// {
//     Service::new(
//         Router::new()
//             .path("/metrics")
//             .post(goal)
//     ).hoop(Logger::new())
// }

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


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let config = Config::load_config();

    let router = load_proxy_config(&config);

    start_http_server(router).await;
}
