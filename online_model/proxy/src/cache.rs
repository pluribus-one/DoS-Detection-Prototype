//! Cache

use moka::sync::Cache;
use salvo::{
    Handler,
    prelude::*
};
use std::time::Duration;
use async_trait::async_trait;
use crate::utils::get_current_time;


/// Data structure to be cached.
#[derive(
    Debug,
    Clone,
)]
pub struct Entry {
    counter     : u32,
    first_seen  : u128,
    last_seen   : u128,
}

impl Entry {
    /// Returns the counter value.
    pub fn counter(&self) -> u32 {
        self.counter
    }

    /// Returns the first seen timestamp.
    pub fn first_seen(&self) -> u128 {
        self.first_seen
    }

    /// Returns the last seen timestamp.
    pub fn last_seen(&self) -> u128 {
        self.last_seen
    }

    /// Increments the counter and update the `last_seen` timestamp.
    pub fn increment_counter(mut self) -> Self {
        self.counter += 1;
        self.last_seen = get_current_time();

        self
    }
}

impl Default for Entry {
    fn default() -> Self {
        let timestamp = get_current_time();
        Self {
            counter     : 1,
            first_seen  : timestamp,
            last_seen   : timestamp,
        }
    }
}

/// A wrapper to `moka::sync::Cache`.
pub struct IpsCache(Cache<String, Entry>);

impl IpsCache {
    /// Create a new `IpsCache`.
    pub fn new(
        tti: u64
    ) -> Self
    {
        Self(
            Cache::builder()
                .time_to_idle(Duration::from_secs(tti))
                .max_capacity(500)
                .build()
        )
    }

    /// Increase or inizialize a counter for a provided IP.
    pub fn increment_counter(
        &self,
        ip: &str
    ) -> moka::Entry<String, Entry>
    {
        self.0
            .entry_by_ref(ip)
            .and_upsert_with(|maybe_entry| {
                if let Some(entry) = maybe_entry {
                    // The entry exists, increment the value.
                    entry.into_value().increment_counter()
                } else {
                    // The entry does not exist, insert a new value.
                    Entry::default()
                }
            })
    }
}

#[async_trait]
impl Handler for IpsCache {
    async fn handle(
        &self,
        req     : &mut Request,
        depot   : &mut Depot,
        res     : &mut Response,
        ctrl    : &mut FlowCtrl
    )
    {
        let remote_addr =
            match req.remote_addr().as_ipv4() {
                Some(ip_address) => ip_address.to_string(),
                None             => req.remote_addr().as_ipv6().unwrap().to_string(),
            };

        let entry = 
            self.increment_counter(
                remote_addr.split(':')
                    .collect::<Vec<&str>>()[0]
            );

        // TODO: It is suboptimal, should be better share the cache
        depot.insert("n_req", entry.value().clone());

        ctrl.call_next(req, depot, res).await;
    }
}
