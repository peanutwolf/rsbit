//! Client configuration for the Bybit SDK.
//!
//! `ClientConfig` mirrors the Python `_V5HTTPManager` dataclass fields,
//! providing all connection and authentication settings.

use std::collections::HashSet;
use std::time::Duration;

/// Known API domains.
pub const DOMAIN_MAIN: &str = "bybit";
pub const DOMAIN_ALT: &str = "bytick";
pub const DOMAIN_TK: &str = "bybit-tr";

/// Known top-level domains.
pub const TLD_MAIN: &str = "com";
pub const TLD_NL: &str = "nl";
pub const TLD_HK: &str = "com.hk";
pub const TLD_KZ: &str = "kz";
pub const TLD_EU: &str = "eu";

/// Subdomain constants.
const SUBDOMAIN_TESTNET: &str = "api-testnet";
const SUBDOMAIN_MAINNET: &str = "api";
const DEMO_SUBDOMAIN_TESTNET: &str = "api-demo-testnet";
const DEMO_SUBDOMAIN_MAINNET: &str = "api-demo";

/// Default Bybit error codes that trigger automatic retry.
fn default_retry_codes() -> HashSet<i32> {
    [10002, 10006, 30034, 30035, 130035, 130150]
        .into_iter()
        .collect()
}

/// Authentication method for API requests.
#[derive(Debug, Clone)]
pub enum AuthMethod {
    /// HMAC-SHA256 authentication (default).
    Hmac,
    /// RSA PKCS1v15 authentication.
    Rsa,
}

/// Configuration for the Bybit HTTP and WebSocket clients.
///
/// Mirrors all fields from the Python `_V5HTTPManager` dataclass.
/// Use the builder methods or `Default` for convenient construction.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Use testnet endpoints.
    pub testnet: bool,
    /// API domain (e.g., "bybit", "bytick", "bybit-tr").
    pub domain: String,
    /// Top-level domain (e.g., "com", "nl", "com.hk").
    pub tld: String,
    /// Use demo trading endpoints.
    pub demo: bool,
    /// Authentication method (HMAC or RSA).
    pub auth_method: AuthMethod,
    /// Bybit API key (None for public-only access).
    pub api_key: Option<String>,
    /// Bybit API secret or RSA private key PEM (None for public-only access).
    pub api_secret: Option<String>,
    /// Log HTTP requests and responses.
    pub log_requests: bool,
    /// HTTP request timeout.
    pub timeout: Duration,
    /// Receive window in milliseconds for signature validation.
    pub recv_window: u64,
    /// Force retry on network errors (timeout, SSL, connection).
    pub force_retry: bool,
    /// Bybit error codes that trigger automatic retry.
    pub retry_codes: HashSet<i32>,
    /// Bybit error codes to silently ignore (do not raise errors).
    pub ignore_codes: HashSet<i32>,
    /// Maximum number of retry attempts.
    pub max_retries: u32,
    /// Delay between retries.
    pub retry_delay: Duration,
    /// Referral ID sent in the Referer header.
    pub referral_id: Option<String>,
    /// Include `response.elapsed` timing in return values.
    pub record_request_time: bool,
    /// Include response headers in return values.
    pub return_response_headers: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            testnet: false,
            domain: DOMAIN_MAIN.to_string(),
            tld: TLD_MAIN.to_string(),
            demo: false,
            auth_method: AuthMethod::Hmac,
            api_key: None,
            api_secret: None,
            log_requests: false,
            timeout: Duration::from_secs(10),
            recv_window: 5000,
            force_retry: false,
            retry_codes: default_retry_codes(),
            ignore_codes: HashSet::new(),
            max_retries: 3,
            retry_delay: Duration::from_secs(3),
            referral_id: None,
            record_request_time: false,
            return_response_headers: false,
        }
    }
}

impl ClientConfig {
    /// Create a config for testnet with API key authentication.
    pub fn testnet(api_key: impl Into<String>, api_secret: impl Into<String>) -> Self {
        Self {
            testnet: true,
            api_key: Some(api_key.into()),
            api_secret: Some(api_secret.into()),
            ..Default::default()
        }
    }

    /// Create a config for mainnet with API key authentication.
    pub fn mainnet(api_key: impl Into<String>, api_secret: impl Into<String>) -> Self {
        Self {
            api_key: Some(api_key.into()),
            api_secret: Some(api_secret.into()),
            ..Default::default()
        }
    }

    /// Build the base HTTP URL from the configured subdomain, domain, and TLD.
    pub fn base_url(&self) -> String {
        let subdomain = match (self.demo, self.testnet) {
            (true, true) => DEMO_SUBDOMAIN_TESTNET,
            (true, false) => DEMO_SUBDOMAIN_MAINNET,
            (false, true) => SUBDOMAIN_TESTNET,
            (false, false) => SUBDOMAIN_MAINNET,
        };
        format!("https://{}.{}.{}", subdomain, self.domain, self.tld)
    }

    /// Set RSA authentication mode.
    pub fn with_rsa_auth(mut self) -> Self {
        self.auth_method = AuthMethod::Rsa;
        self
    }

    /// Set the domain.
    pub fn with_domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = domain.into();
        self
    }

    /// Set the TLD.
    pub fn with_tld(mut self, tld: impl Into<String>) -> Self {
        self.tld = tld.into();
        self
    }

    /// Enable demo mode.
    pub fn with_demo(mut self) -> Self {
        self.demo = true;
        self
    }

    /// Set the request timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the receive window in milliseconds.
    pub fn with_recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }

    /// Enable forced retry on network errors.
    pub fn with_force_retry(mut self) -> Self {
        self.force_retry = true;
        self
    }

    /// Set the maximum number of retries.
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Set the delay between retries.
    pub fn with_retry_delay(mut self, retry_delay: Duration) -> Self {
        self.retry_delay = retry_delay;
        self
    }

    /// Set a referral ID.
    pub fn with_referral_id(mut self, referral_id: impl Into<String>) -> Self {
        self.referral_id = Some(referral_id.into());
        self
    }

    /// Enable request time recording.
    pub fn with_record_request_time(mut self) -> Self {
        self.record_request_time = true;
        self
    }

    /// Enable response header inclusion in results.
    pub fn with_return_response_headers(mut self) -> Self {
        self.return_response_headers = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_base_url() {
        let config = ClientConfig::default();
        assert_eq!(config.base_url(), "https://api.bybit.com");
    }

    #[test]
    fn testnet_config_base_url() {
        let config = ClientConfig::testnet("key", "secret");
        assert_eq!(config.base_url(), "https://api-testnet.bybit.com");
        assert_eq!(config.api_key.as_deref(), Some("key"));
    }

    #[test]
    fn demo_testnet_base_url() {
        let config = ClientConfig::testnet("k", "s").with_demo();
        assert_eq!(config.base_url(), "https://api-demo-testnet.bybit.com");
    }

    #[test]
    fn demo_mainnet_base_url() {
        let config = ClientConfig::mainnet("k", "s").with_demo();
        assert_eq!(config.base_url(), "https://api-demo.bybit.com");
    }

    #[test]
    fn custom_domain_and_tld() {
        let config = ClientConfig::default()
            .with_domain(DOMAIN_ALT)
            .with_tld(TLD_HK);
        assert_eq!(config.base_url(), "https://api.bytick.com.hk");
    }

    #[test]
    fn default_retry_codes_populated() {
        let config = ClientConfig::default();
        assert!(config.retry_codes.contains(&10002));
        assert!(config.retry_codes.contains(&10006));
        assert!(config.retry_codes.contains(&130035));
        assert_eq!(config.retry_codes.len(), 6);
    }

    #[test]
    fn builder_methods_chain() {
        let config = ClientConfig::mainnet("k", "s")
            .with_rsa_auth()
            .with_force_retry()
            .with_max_retries(5)
            .with_timeout(Duration::from_secs(30))
            .with_recv_window(8000)
            .with_referral_id("my-ref");

        assert!(matches!(config.auth_method, AuthMethod::Rsa));
        assert!(config.force_retry);
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.recv_window, 8000);
        assert_eq!(config.referral_id.as_deref(), Some("my-ref"));
    }
}
