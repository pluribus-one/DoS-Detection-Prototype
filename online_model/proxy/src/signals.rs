//! A module to handle shutdown signals.

use salvo::server::ServerHandle;
use tokio::signal;


/// Listen shutdown signal.
pub async fn listen_shutdown_signal(
    internal_handle: ServerHandle,
    proxy_handle: ServerHandle
)
{
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(windows)]
    let terminate = async {
        signal::windows::signal(signal::windows::Signal::ctrl_c())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => println!("CTRL+C signal received"),
        _ = terminate => println!("SIGTERM received"),
    };

    internal_handle.stop_graceful(None);
    proxy_handle.stop_graceful(None);
}
