//! A module defining a `SlidingGuard` with multiples quotas.

use time::OffsetDateTime;
use salvo::rate_limiter::BasicQuota;


/// A data model defining a fac-simile `FixedGuard` with
/// multiples quotas.
#[derive(
    Clone,
    Debug
)]
pub struct MultiGuard {
    resets : Vec<OffsetDateTime>,
    counts : Vec<usize>,
    quotas : Vec<Option<BasicQuota>>,
}

impl Default for MultiGuard {
    fn default() -> Self {
        Self::new(1)
    }
}

impl MultiGuard {
    /// Create a new `MultiGuard` with a specific `size` (number of metrics).
    pub fn new(
        size: usize
    ) -> Self
    {
        Self {
            resets : vec![OffsetDateTime::now_utc(); size],
            counts : vec![0; size],
            quotas : vec![None; size],
        }
    }

    /// Check whether a client has exceeded the allowed number of requests,
    /// in other words, it tries to detect DoS/Bot attacks.
    pub async fn verify(
        &mut self,
        quotas: &[BasicQuota]
    ) -> bool
    {
        quotas
            .iter()
            .enumerate()
            .map(|(idx, quota)| self.single_verify(idx, quota))
            .all(|res| res)
    }

    /// Perform the check for a single `BasicQuota`.
    fn single_verify(
        &mut self,
        idx     : usize,
        quota   : &BasicQuota
    ) -> bool
    {
        if self.quotas[idx].is_none() || OffsetDateTime::now_utc() > self.resets[idx]
            || self.quotas[idx].as_ref() != Some(quota)
        {
            if self.quotas[idx].as_ref() != Some(quota) {
                let mut quota = quota.clone();
                if quota.limit == 0 {
                    quota.limit = 1;
                }
                self.quotas[idx] = Some(quota);
            }
            self.resets[idx] = OffsetDateTime::now_utc() + quota.period;
            self.counts[idx] = 1;
            true
        } else if self.counts[idx] < quota.limit {
            self.counts[idx] += 1;
            true
        } else {
            false
        }
    }
}
