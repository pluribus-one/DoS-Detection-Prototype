mod data;

use moka::{
    sync::Cache,
    Entry
};
use data::Data;
use futures::future;
use std::time::Duration;


fn increment_counter(
    cache: &Cache<String, Data>,
    key: &str
) -> Entry<String, Data>
{
    cache
        .entry_by_ref(key)
        .and_upsert_with(|maybe_entry| {
            if let Some(entry) = maybe_entry {
                // The entry exists, increment the value.
                entry.into_value().increment_counter()
            } else {
                // The entry does not exist, insert a new value.
                Data::new()
            }
        })
}

#[tokio::main]
async fn main() {
    let cache: Cache<String, Data> =
        Cache::builder()
            .time_to_idle(Duration::from_secs(60))
            .max_capacity(2000)
            .build();

    increment_counter(&cache, "127.0.0.1");

    let tasks: Vec<_> =
        (1..=100_000_000).map(|_| {
            let cloned_cache = cache.clone();

            tokio::spawn(async move {
                increment_counter(&cloned_cache, "127.0.0.1");
            })
        }).collect();

    future::join_all(tasks).await;

    println!("{:?}", cache);
}
