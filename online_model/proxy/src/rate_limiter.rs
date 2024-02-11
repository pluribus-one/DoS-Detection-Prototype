pub mod multi_sliding_guard;
pub mod metrics;
pub mod cache;

pub use multi_sliding_guard::MultiSlidingGuard;
pub use metrics::{
    QuotaMetrics,
    Metrics
};
pub use cache::Cache;

use tokio::sync::mpsc::{
    UnboundedSender,
    UnboundedReceiver,
    self
};
use salvo::{
    handler::{
        Skipper,
        none_skipper
    },
    prelude::*,
    rate_limiter::{
        RateIssuer,
        RemoteIpIssuer
    }
};


// `RateLimiter` handle used to update metrics.
#[derive(Clone)]
pub struct RateLimiterHandler {
    tx_update: UnboundedSender<Metrics>,
}

impl RateLimiterHandler {
    /// Send a update to the RateLimiter core.
    pub fn update_metrics(
        &self,
        metrics: Metrics
    )
    {
        self.tx_update.send(metrics);
    }
}

/// A structure modelling a rate limiter to defend against
/// Bot and Dos attacks.
pub struct RateLimiter {
    guard       : MultiSlidingGuard,
    store       : Cache,
    issuer      : RemoteIpIssuer,
    // quota_getter: Option<QuotaMetrics>,
    skipper     : Box<dyn Skipper>,
    // tx_update   : UnboundedSender<Metrics>,
    // rx_update   : UnboundedReceiver<Metrics>
}

impl RateLimiter {
    /// Create a new `RateLimiter`
    pub fn new(
        guard       : MultiSlidingGuard,
        store       : Cache,
        issuer      : RemoteIpIssuer,
        // quota_getter: Option<QuotaMetrics>
    ) -> Self
    {
        // let (tx_update, rx_update) = mpsc::unbounded_channel();

        Self {
            guard,
            store,
            issuer,
            // quota_getter,
            skipper: Box::new(none_skipper),
            // tx_update,
            // rx_update
        }
    }

    /// Sets skipper and returns new `RateLimiter`.
    pub fn with_skipper(
        mut self,
        skipper: impl Skipper
    ) -> Self
    {
        self.skipper = Box::new(skipper);
        self
    }

    // Get a `RateLimiterHandler` to update the metrics.
    // pub fn handle(
    //     &self
    // ) -> RateLimiterHandler
    // {
    //     RateLimiterHandler {
    //         tx_update: self.tx_update.clone()
    //     }
    // }
}

use crate::TEST;

#[async_trait]
impl Handler for RateLimiter
{
    async fn handle(
        &self,
        req     : &mut Request,
        depot   : &mut Depot,
        res     : &mut Response,
        ctrl    : &mut FlowCtrl
    )
    {
        if self.skipper.skipped(req, depot) {
            return;
        }

        if TEST.read().await.is_none() {
            return;
        }

        // if self.quota_getter.is_none() {
        //     return;
        // }

        let key =
            match self.issuer.issue(req, depot).await {
                Some(key) => key,
                None => {
                    res.render(StatusError::bad_request().brief("Malformed request"));
                    ctrl.skip_rest();
                    return;
                }
            };

        let quota = TEST.read().await;

        let mut guard =
            match self.store.load_guard(&key, &self.guard).await {
                Ok(guard) => guard,
                Err(e) => {
                    tracing::error!(error = ?e, "RateLimiter error: {}", e);
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    ctrl.skip_rest();
                    return;
                }
            };

        let verified = guard.verify(quota.as_ref().unwrap().get_metrics()).await;

        if !verified {
            res.status_code(StatusCode::TOO_MANY_REQUESTS);
            ctrl.skip_rest();
        }

        if let Err(e) = self.store.save_guard(key, guard).await {
            tracing::error!(error = ?e, "RateLimiter save guard failed");
        }
    }
}
