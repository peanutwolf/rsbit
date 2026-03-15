//! Unified HTTP client — mirrors Python's `unified_trading.HTTP`.
//!
//! `HTTP` is a thin wrapper around [`HttpManager`] that provides a
//! user-friendly constructor and exposes all endpoint methods via `Deref`.

use std::ops::Deref;

use crate::config::ClientConfig;
use crate::error::Result;
use super::HttpManager;

/// Unified Bybit V5 HTTP client.
///
/// Wraps [`HttpManager`] and exposes every endpoint method (market, trade,
/// account, position, asset, …) through `Deref`.  This is the primary
/// entry-point for users of the SDK — equivalent to Python's
/// `pybit.unified_trading.HTTP`.
///
/// # Examples
///
/// ```rust,no_run
/// use rsbit::{HTTP, ClientConfig};
///
/// # async fn run() -> rsbit::error::Result<()> {
/// // Public (unauthenticated) client
/// let client = HTTP::new(ClientConfig::default())?;
///
/// // Authenticated client
/// let client = HTTP::new(ClientConfig::testnet("key", "secret"))?;
///
/// // All HttpManager methods are available directly:
/// let kline = client.get_kline(Default::default()).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct HTTP {
    manager: HttpManager,
}

impl HTTP {
    /// Create a new unified HTTP client from the given config.
    pub fn new(config: ClientConfig) -> Result<Self> {
        Ok(Self {
            manager: HttpManager::new(config)?,
        })
    }

    /// Access the underlying `HttpManager`.
    pub fn manager(&self) -> &HttpManager {
        &self.manager
    }
}

impl Deref for HTTP {
    type Target = HttpManager;

    fn deref(&self) -> &HttpManager {
        &self.manager
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_deref_gives_access_to_manager_methods() {
        let client = HTTP::new(ClientConfig::default()).unwrap();
        // Deref gives us HttpManager methods directly
        assert_eq!(client.endpoint(), "https://api.bybit.com");
        assert!(!client.config().testnet);
    }

    #[test]
    fn http_testnet() {
        let client = HTTP::new(ClientConfig::testnet("k", "s")).unwrap();
        assert_eq!(client.endpoint(), "https://api-testnet.bybit.com");
    }
}
