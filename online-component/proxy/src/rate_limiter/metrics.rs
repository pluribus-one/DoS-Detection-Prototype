//! A module defining the metrics of `RateLimiter` mechanism.

use salvo::{
    prelude::*,
    rate_limiter::BasicQuota
};
use tokio::sync::RwLock;
use once_cell::sync::Lazy;
use serde::Deserialize;


/// A set of constants defining the last in seconds of each available window.
const N_WINDOWS: usize        = 2;
const FIRST_WINDOW_LAST: i64  = 10;
const SECOND_WINDOW_LAST: i64 = 100;
// const THIRD_WINDOW_LAST: i64  = 1000;

/// A data structure in charge of globally mantaining
/// the current values of metrics. This is behind a `RwLock` but
/// should not be heavy since only one component, it is able to
/// update them.
pub static SYSTEM_METRICS: Lazy<RwLock<Option<QuotaMetrics>>> =
    Lazy::new(|| { RwLock::new(None) });


/// A structure defined to map the `/metrics` API body.
#[derive(
    Deserialize,
    Extractible,
    Debug
)]
#[salvo(extract(
    default_source(
        from  = "body",
        parse = "json"
    )
))]
pub struct Metrics {
    pub count_first_window  : usize,
    pub count_second_window : usize,
}

impl Metrics {
    /// Return the value of the first window.
    pub fn count_first_window(
        &self
    ) -> usize
    {
        self.count_first_window
    }

    /// Return the value of the second window.
    pub fn count_second_window(
        &self
    ) -> usize
    {
        self.count_second_window
    }
}


/// A data structure modelling the metrics' shape within
/// the `Cache`. It is assigned to each IP address
/// (client) and it is in charge to keeping real time
/// counts for each client.
#[derive(Debug)]
pub struct QuotaMetrics {
    quotas: [BasicQuota; N_WINDOWS]
}

impl QuotaMetrics {
    /// Create a new `QuotaMetrics` consuming a `Metrics`.
    pub fn new(
        metrics: Metrics
    ) -> Self
    {
        Self {
            quotas: [
                BasicQuota::set_seconds(
                    metrics.count_first_window(),
                    FIRST_WINDOW_LAST
                ),
                BasicQuota::set_seconds(
                    metrics.count_second_window(),
                    SECOND_WINDOW_LAST
                )
            ]
        }
    }

    /// Update the limits inside a `QuotaMetrics` consuming a `Metrics`.
    pub fn update_metrics(
        &mut self,
        metrics: Metrics
    )
    {
        if self.quotas[0].limit != metrics.count_first_window() {
            self.quotas[0].limit = metrics.count_first_window()
        }

        if self.quotas[1].limit != metrics.count_second_window() {
            self.quotas[1].limit = metrics.count_second_window()
        }
    }

    /// Return a reference to the current `BasicQuota`.
    pub fn get_metrics(
        &self
    ) -> &[BasicQuota; N_WINDOWS]
    {
        &self.quotas
    }
}
