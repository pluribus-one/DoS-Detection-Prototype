//! A module to handle proxy configuration.

use serde::Deserialize;


/// A data structure mapping TOML configuration.
#[derive(
    Deserialize,
    Debug
)]
pub struct Config {
    tls             : bool,
    upstream        : String,
    ssl_certificate : String,
    ssl_key         : String,
}

impl Config {
    /// Load configuration from TOML file.
    pub fn load_config() -> Self
    {
        std::fs::read_to_string("./settings.toml")
            .ok()
            .and_then(|content| toml::from_str(&content).ok())
            .unwrap()
    }

    /// Return `tls` value.
    pub fn tls(&self) -> bool
    {
        self.tls
    }

    /// Return a ref to `upstream` value.
    pub fn upstream(&self) -> &str
    {
        &self.upstream
    }

    /// Return a ref to `ssl_certificate` value.
    pub fn ssl_certificate(&self) -> &str
    {
        &self.ssl_certificate
    }

    /// Return a ref to `ssl_key` value.
    pub fn ssl_key(&self) -> &str
    {
        &self.ssl_key
    }
}

