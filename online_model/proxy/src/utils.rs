//! A module with util functions

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
