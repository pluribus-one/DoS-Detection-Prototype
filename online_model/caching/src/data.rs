use std::time::{
    SystemTime,
    UNIX_EPOCH
};


/// Returns the current time in milliseconds.
pub fn get_current_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

/// Data structure to be cached.
#[derive(
    Debug,
    Clone,
)]
pub struct Data {
    counter     : u32,
    first_seen  : u128,
    last_seen   : u128,
}

impl Data {
    pub fn new() -> Data
    {
        let timestamp = get_current_time();
        Data {
            counter     : 1,
            first_seen  : timestamp,
            last_seen   : timestamp,
        }
    }

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
