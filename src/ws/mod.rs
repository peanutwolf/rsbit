//! WebSocket client for the Bybit V5 API.
//!
//! Mirrors Python's `_websocket_stream.py` — connection management,
//! ping/pong, authentication, subscription tracking, and message dispatch.

pub mod delta;
pub mod manager;
pub mod streams;
pub mod subscription;
pub mod trading;

pub use manager::{WsConfig, WsManager};
pub use trading::WsTradingManager;

use crate::config::ClientConfig;

/// WS subdomain constants (different from HTTP's `api-*` subdomains).
const WS_SUBDOMAIN_MAINNET: &str = "stream";
const WS_SUBDOMAIN_TESTNET: &str = "stream-testnet";
const WS_DEMO_SUBDOMAIN_MAINNET: &str = "stream-demo";
const WS_DEMO_SUBDOMAIN_TESTNET: &str = "stream-demo-testnet";

/// Build a WebSocket URL from the client config and channel type.
///
/// Mirrors the Python URL construction in `_websocket_stream.py` and
/// `unified_trading.py`.
///
/// # Channel types
/// - `"linear"`, `"inverse"`, `"spot"`, `"option"` — public market data
/// - `"private"` — authenticated private channel (uses config TLD)
/// - `"trade"` — authenticated trading channel (uses config TLD)
///
/// # URL patterns
/// - Public: `wss://{subdomain}.{domain}.com/v5/public/{channel_type}`
/// - Private: `wss://{subdomain}.{domain}.{tld}/v5/private`
/// - Trade: `wss://{subdomain}.{domain}.{tld}/v5/trade`
pub fn ws_url(config: &ClientConfig, channel_type: &str) -> String {
    let subdomain = match (config.demo, config.testnet) {
        (true, true) => WS_DEMO_SUBDOMAIN_TESTNET,
        (true, false) => WS_DEMO_SUBDOMAIN_MAINNET,
        (false, true) => WS_SUBDOMAIN_TESTNET,
        (false, false) => WS_SUBDOMAIN_MAINNET,
    };

    if channel_type == "private" || channel_type == "trade" {
        format!(
            "wss://{}.{}.{}/v5/{}",
            subdomain, config.domain, config.tld, channel_type
        )
    } else {
        // Public channels always use .com TLD (mirrors Python behavior)
        format!(
            "wss://{}.{}.com/v5/public/{}",
            subdomain, config.domain, channel_type
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{DOMAIN_ALT, TLD_HK};

    #[test]
    fn public_linear_mainnet() {
        let config = ClientConfig::default();
        assert_eq!(
            ws_url(&config, "linear"),
            "wss://stream.bybit.com/v5/public/linear"
        );
    }

    #[test]
    fn public_spot_testnet() {
        let config = ClientConfig::testnet("k", "s");
        assert_eq!(
            ws_url(&config, "spot"),
            "wss://stream-testnet.bybit.com/v5/public/spot"
        );
    }

    #[test]
    fn public_option_mainnet() {
        let config = ClientConfig::default();
        assert_eq!(
            ws_url(&config, "option"),
            "wss://stream.bybit.com/v5/public/option"
        );
    }

    #[test]
    fn public_inverse_mainnet() {
        let config = ClientConfig::default();
        assert_eq!(
            ws_url(&config, "inverse"),
            "wss://stream.bybit.com/v5/public/inverse"
        );
    }

    #[test]
    fn private_mainnet() {
        let config = ClientConfig::mainnet("k", "s");
        assert_eq!(
            ws_url(&config, "private"),
            "wss://stream.bybit.com/v5/private"
        );
    }

    #[test]
    fn private_testnet() {
        let config = ClientConfig::testnet("k", "s");
        assert_eq!(
            ws_url(&config, "private"),
            "wss://stream-testnet.bybit.com/v5/private"
        );
    }

    #[test]
    fn private_custom_tld() {
        let config = ClientConfig::mainnet("k", "s").with_tld(TLD_HK);
        assert_eq!(
            ws_url(&config, "private"),
            "wss://stream.bybit.com.hk/v5/private"
        );
    }

    #[test]
    fn public_ignores_custom_tld() {
        // Public channels always use .com, even with custom TLD
        let config = ClientConfig::default().with_tld(TLD_HK);
        assert_eq!(
            ws_url(&config, "linear"),
            "wss://stream.bybit.com/v5/public/linear"
        );
    }

    #[test]
    fn demo_mainnet() {
        let config = ClientConfig::mainnet("k", "s").with_demo();
        assert_eq!(
            ws_url(&config, "linear"),
            "wss://stream-demo.bybit.com/v5/public/linear"
        );
        assert_eq!(
            ws_url(&config, "private"),
            "wss://stream-demo.bybit.com/v5/private"
        );
    }

    #[test]
    fn demo_testnet() {
        let config = ClientConfig::testnet("k", "s").with_demo();
        assert_eq!(
            ws_url(&config, "spot"),
            "wss://stream-demo-testnet.bybit.com/v5/public/spot"
        );
        assert_eq!(
            ws_url(&config, "private"),
            "wss://stream-demo-testnet.bybit.com/v5/private"
        );
    }

    #[test]
    fn alt_domain() {
        let config = ClientConfig::default().with_domain(DOMAIN_ALT);
        assert_eq!(
            ws_url(&config, "linear"),
            "wss://stream.bytick.com/v5/public/linear"
        );
    }

    #[test]
    fn spread_channel() {
        let config = ClientConfig::default();
        assert_eq!(
            ws_url(&config, "spread"),
            "wss://stream.bybit.com/v5/public/spread"
        );
    }

    #[test]
    fn trade_mainnet() {
        let config = ClientConfig::mainnet("k", "s");
        assert_eq!(
            ws_url(&config, "trade"),
            "wss://stream.bybit.com/v5/trade"
        );
    }

    #[test]
    fn trade_testnet() {
        let config = ClientConfig::testnet("k", "s");
        assert_eq!(
            ws_url(&config, "trade"),
            "wss://stream-testnet.bybit.com/v5/trade"
        );
    }

    #[test]
    fn trade_custom_tld() {
        let config = ClientConfig::mainnet("k", "s").with_tld(TLD_HK);
        assert_eq!(
            ws_url(&config, "trade"),
            "wss://stream.bybit.com.hk/v5/trade"
        );
    }
}
