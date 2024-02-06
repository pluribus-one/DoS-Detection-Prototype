use salvo::prelude::*;
use serde::Deserialize;


pub struct Metrics {
    count_first_window: u64,
    count_second_window: u64,
    count_thrid_window: u64,
}


