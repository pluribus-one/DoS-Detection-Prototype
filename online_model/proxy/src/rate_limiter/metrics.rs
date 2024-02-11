//! 

use salvo::{
    prelude::*,
    rate_limiter::CelledQuota
};
use std::marker::Send;
use serde::Deserialize;


/// A structure defined to map the `/metrics` API body.
#[derive(Deserialize, Extractible, Debug)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct Metrics {
    pub count_first_window  : usize,
    pub count_second_window : usize,
    pub count_third_window  : usize,
}

impl Metrics {
    pub fn count_first_window(&self) -> usize
    {
        self.count_first_window
    }

    pub fn count_second_window(&self) -> usize
    {
        self.count_second_window
    }

    pub fn count_third_window(&self) -> usize
    {
        self.count_third_window
    }
}

///
#[derive(Debug)]
pub struct QuotaMetrics {
    quotas: [CelledQuota; 3]
}

impl QuotaMetrics {
    ///
    pub fn new(
        metrics: Metrics
    ) -> Self
    {
        Self {
            quotas: [
                CelledQuota::set_seconds(
                    metrics.count_first_window(), 1, 10
                ),
                CelledQuota::set_seconds(
                    metrics.count_second_window(), 1, 100
                ),
                CelledQuota::set_seconds(
                    metrics.count_third_window(), 1, 1000
                )
            ]
        }
    }

    ///
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

        if self.quotas[2].limit != metrics.count_third_window() {
            self.quotas[2].limit = metrics.count_third_window()
        }
    }

    ///
    pub fn get_metrics(
        &self
    ) -> &[CelledQuota; 3]
    {
        &self.quotas
    }
}

