pub mod api;
pub mod rate_limiter;

use salvo::{
    prelude::*,
    rate_limiter::RemoteIpIssuer,
    proxy::Proxy
};

use rate_limiter::{
    MultiSlidingGuard,
    RateLimiter,
    Cache
};
use api::update_metrics;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let limiter =
        RateLimiter::new(
            MultiSlidingGuard::new(3),
            Cache::default(),
            RemoteIpIssuer,
        );

    let internal_router =
        Router::with_path("/metrics")
            .goal(update_metrics);

    let proxy_router =
        Router::with_path("/<**rest>")
            .hoop(limiter)
            .goal(
                Proxy::default_hyper_client("https://www.rust-lang.org")
            );


    let internal_acceptor =
        TcpListener::new("0.0.0.0:5800")
            .bind();

    let proxy_acceptor =
        TcpListener::new("0.0.0.0:5801")
            .bind();

    tokio::try_join!(
        Server::new(internal_acceptor.await)
            .try_serve(internal_router),
        Server::new(proxy_acceptor.await)
            .try_serve(proxy_router),
    )
    .unwrap();
 
}
