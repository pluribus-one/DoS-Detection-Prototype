//! A module implementing a concurrent and lock free Cache.

use std::{
    convert::Infallible,
    time::Duration
};
use moka::future::Cache as MokaCache;
use super::multi_sliding_guard::MultiSlidingGuard;


/// Default cache parameters.
const DEFAULT_SIZE: u64 = 3000;
const DEFAULT_TTI : u64 = 60;


/// A data model defining a concurrent and lock free Cache.
#[derive(Debug)]
pub struct Cache {
    inner: MokaCache<String, MultiSlidingGuard>,
}

impl<'a> Default for Cache
{
    fn default() -> Self
    {
        Self::new(
            DEFAULT_SIZE,
            DEFAULT_TTI
        )
    }
}

impl Cache {
    /// Create a new `Cache`.
    pub fn new(
        capacity: u64,
        tti     : u64
    ) -> Self
    {
        Self {
            inner: MokaCache::builder()
                .time_to_idle(Duration::from_secs(tti))
                .max_capacity(capacity)
                .build()
        }
    }

    /// Load a guard from the cache, if it is not present return
    /// the provided one.
    pub async fn load_guard(
        &self,
        key     : &str,
        refer   : &MultiSlidingGuard
    ) -> Result<MultiSlidingGuard, Infallible>
    {
        let guard = self.inner.get(key).await;
        if let Some(guard) = guard {
            Ok(guard)
        } else {
            Ok(refer.clone())
        }
    }

    /// Save/Update a guard into the cache.
    pub async fn save_guard(
        &self,
        key     : String,
        guard   : MultiSlidingGuard
    ) -> Result<(), Infallible>
    {
        self.inner.insert(key, guard).await;
        Ok(())
    }
}
