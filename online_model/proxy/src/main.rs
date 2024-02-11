pub mod api;
pub mod rate_limiter;

use tokio::sync::RwLock;
use once_cell::sync::Lazy;
use salvo::prelude::*;
use salvo::rate_limiter::RemoteIpIssuer;

use rate_limiter::{
    MultiSlidingGuard,
    RateLimiter,
    QuotaMetrics,
    Cache
};
use api::{
    home,
    update_metrics
};


pub static TEST: Lazy<RwLock<Option<QuotaMetrics>>> = Lazy::new(|| { RwLock::new(None) });


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let limiter = 
        RateLimiter::new(
            MultiSlidingGuard::new(3),
            Cache::default(),
            RemoteIpIssuer,
        );

    let router = 
        Router::new()
            .push(
                Router::new()
                    .get(home)
                    .hoop(limiter)
            )
            .push(
                Router::new()
                    .path("/metrics")
                    .post(update_metrics)
            );

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;

    Server::new(acceptor).serve(router).await;
 }
