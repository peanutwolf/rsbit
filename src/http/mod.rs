//! HTTP client for the Bybit V5 API.
//!
//! `HttpManager` wraps a `reqwest::Client` and provides the core
//! `submit_request` method that all endpoint modules delegate to.

pub mod client;
mod request;
mod retry;

pub use request::Method;

use std::collections::HashMap;
use std::time::Duration;

use reqwest::Client;
use serde_json::Value;

use crate::config::ClientConfig;
use crate::error::{BybitError, Result};

/// Response from the Bybit API, optionally including timing and headers.
#[derive(Debug)]
pub struct ApiResponse {
    /// The parsed JSON body.
    pub body: Value,
    /// Request elapsed time (only populated if `record_request_time` is set).
    pub elapsed: Option<Duration>,
    /// Response headers (only populated if `return_response_headers` is set).
    pub headers: Option<HashMap<String, String>>,
}

/// The core HTTP manager that all endpoint methods use.
///
/// Mirrors Python's `_V5HTTPManager` — holds a `reqwest::Client`, the
/// base URL, and config for auth/retry/logging.
#[derive(Debug, Clone)]
pub struct HttpManager {
    client: Client,
    config: ClientConfig,
    endpoint: String,
}

impl HttpManager {
    /// Create a new `HttpManager` from a `ClientConfig`.
    pub fn new(config: ClientConfig) -> Result<Self> {
        let endpoint = config.base_url();

        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        default_headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        if let Some(ref referral_id) = config.referral_id {
            default_headers.insert(
                reqwest::header::REFERER,
                reqwest::header::HeaderValue::from_str(referral_id)
                    .map_err(|_| BybitError::FailedRequest {
                        request: "HttpManager::new".into(),
                        message: "Invalid referral_id header value".into(),
                        status_code: 0,
                        time: String::new(),
                        resp_headers: None,
                    })?,
            );
        }

        let client = Client::builder()
            .timeout(config.timeout)
            .default_headers(default_headers)
            .build()?;

        tracing::debug!("Initializing HTTP session for {}", endpoint);

        Ok(Self {
            client,
            config,
            endpoint,
        })
    }

    /// Get a reference to the underlying config.
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// Get the base endpoint URL.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_manager_default_config() {
        let config = ClientConfig::default();
        let manager = HttpManager::new(config).unwrap();
        assert_eq!(manager.endpoint(), "https://api.bybit.com");
    }

    #[test]
    fn new_manager_testnet() {
        let config = ClientConfig::testnet("key", "secret");
        let manager = HttpManager::new(config).unwrap();
        assert_eq!(manager.endpoint(), "https://api-testnet.bybit.com");
    }
}
