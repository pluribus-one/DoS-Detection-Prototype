//! A module defining a `SlidingGuard` with multiples quotas.

use time::{
    Duration,
    OffsetDateTime
};
use salvo::rate_limiter::CelledQuota;


/// A data model defining a fac-simile `SlidingGuard` with
/// multiples quotas.
#[derive(Clone, Debug)]
pub struct MultiSlidingGuard {
    cell_inst   : Vec<OffsetDateTime>,
    cell_span   : Vec<Duration>,
    counts      : Vec<Vec<usize>>,
    head        : Vec<usize>,
    quota       : Vec<Option<CelledQuota>>,
}

impl Default for MultiSlidingGuard {
    fn default() -> Self {
        Self::new(1)
    }
}

impl MultiSlidingGuard {
    /// Create a new `MultiSlidingGuard`.
    pub fn new(
        size: usize
    ) -> Self
    {
        Self {
            cell_inst   : vec![OffsetDateTime::now_utc(); size],
            cell_span   : vec![Duration::default(); size],
            counts      : vec![vec![]; size],
            head        : vec![0; size],
            quota       : vec![None; size],
        }
    }
    
    /// Check whether a client has exceeded the allowed number of requests,
    /// in other words, it tries to detect DoS/Bot attacks.
    pub async fn verify(
        &mut self,
        quotas: &[CelledQuota]
    ) -> bool
    {
        quotas
            .iter()
            .enumerate()
            .map(|(idx, quota)| self.single_verify(idx, &quota))
            .all(|res| res)
    }

    /// Perform the check for a single `CelledQuota`.
    fn single_verify(
        &mut self,
        idx     : usize,
        quota   : &CelledQuota
    ) -> bool
    {
        if self.quota[idx].is_none() || self.quota[idx].as_ref() != Some(quota) {
            let mut quota = quota.clone();
            if quota.limit == 0 {
                quota.limit = 1;
            }
            if quota.cells == 0 {
                quota.cells = 1;
            }
            if quota.cells > quota.limit {
                quota.cells = quota.limit;
            }
            self.cell_inst[idx] = OffsetDateTime::now_utc();
            self.cell_span[idx] = quota.period / (quota.cells as u32);
            self.counts[idx]    = vec![0; quota.cells];
            self.head[idx]      = 0;
            self.counts[idx][0] = 1;
            self.quota[idx]     = Some(quota);
            return true;
        }

        let mut delta = OffsetDateTime::now_utc() - self.cell_inst[idx];

        if delta > quota.period {
            self.counts[idx]    = vec![0; quota.cells];
            self.head[idx]      = 0;
            self.counts[idx][0] = 1;
            self.cell_inst[idx] = OffsetDateTime::now_utc();
            return true;
        } else {
            while delta > self.cell_span[idx] {
                delta -= self.cell_span[idx];
                self.head[idx] = (self.head[idx] + 1) % self.counts[idx].len();
                self.counts[idx][self.head[idx]] = 0;
            }

            self.head[idx] = (self.head[idx] + 1) % self.counts[idx].len();
            self.counts[idx][self.head[idx]] += 1;
            self.cell_inst[idx] = OffsetDateTime::now_utc();
        }

        self.counts[idx]
            .iter()
            .cloned()
            .sum::<usize>() <= quota.limit
    }
}
