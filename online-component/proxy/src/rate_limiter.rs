//! A module defining the `ReteLimiter` component.

pub mod multi_guard;
pub mod metrics;
pub mod cache;

pub use multi_guard::MultiGuard;
pub use metrics::{
    QuotaMetrics,
    Metrics,
    SYSTEM_METRICS
};
pub use cache::Cache;

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


/// A structure modelling a rate limiter to defend against
/// Bot and Dos attacks.
pub struct RateLimiter {
    guard       : MultiGuard,
    store       : Cache,
    issuer      : RemoteIpIssuer,
    skipper     : Box<dyn Skipper>,
}

impl RateLimiter {
    /// Create a new `RateLimiter`
    pub fn new(
        guard       : MultiGuard,
        store       : Cache,
        issuer      : RemoteIpIssuer,
    ) -> Self
    {
        Self {
            guard,
            store,
            issuer,
            skipper: Box::new(none_skipper),
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
}


/// Implementation of `Handle` trait in order to by
/// compliant with Salvo middlewares.
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

        if SYSTEM_METRICS.read().await.is_none() {
            return;
        }

        let key =
            match self.issuer.issue(req, depot).await {
                Some(key) => key,
                None => {
                    res.render(
                        StatusError::bad_request()
                            .brief("Malformed request")
                    );
                    ctrl.skip_rest();
                    return;
                }
            };

        let quota = SYSTEM_METRICS.read().await;

        let mut guard =
            match self.store.load_guard(&key, &self.guard).await {
                Ok(guard)   => guard,
                Err(reason) => {
                    tracing::error!(
                        error = ?reason,
                        "RateLimiter: {}",
                        reason
                    );

                    res.status_code(
                        StatusCode::INTERNAL_SERVER_ERROR
                    );
                    ctrl.skip_rest();

                    return;
                }
            };

        if let Some(quota) = quota.as_ref() {
            let verified = guard.verify(quota.get_metrics()).await;

            if !verified {
                res.status_code(StatusCode::TOO_MANY_REQUESTS);
                ctrl.skip_rest();
            }

            if let Err(reason) = self.store.save_guard(key, guard).await {
                tracing::error!(
                    error = ?reason,
                    "RateLimiter: Failed to save guard"
                );
            }
        } else {
            tracing::error!(
                "RateLimiter: Failed to retrive system metrics"
            )
        }
    }
}
