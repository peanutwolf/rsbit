//! Utility functions for the Bybit SDK.
//!
//! Mirrors the Python `_helpers.py` and `helpers.py` modules.

use std::time::{SystemTime, UNIX_EPOCH};

/// Return the current Unix timestamp in milliseconds.
///
/// Equivalent to Python's `int(time.time() * 10**3)`.
pub fn generate_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before Unix epoch")
        .as_millis() as u64
}

/// Check if a symbol represents an inverse contract (e.g., "BTCUSD", "BTCUSDH24").
///
/// Inverse contracts end with "USD" optionally followed by a futures month code
/// (H, M, U, Z) and a two-digit year.
pub fn is_inverse_contract(symbol: &str) -> bool {
    // Matches: USD at end, or USD followed by [HMUZ]\d\d at end
    let bytes = symbol.as_bytes();
    let len = bytes.len();

    if len >= 3 && &symbol[len - 3..] == "USD" {
        return true;
    }

    if len >= 6 {
        let suffix = &symbol[len - 6..];
        if &suffix[..3] == "USD"
            && matches!(suffix.as_bytes()[3], b'H' | b'M' | b'U' | b'Z')
            && suffix.as_bytes()[4].is_ascii_digit()
            && suffix.as_bytes()[5].is_ascii_digit()
        {
            return true;
        }
    }

    false
}

/// Check if a symbol is a USDT perpetual (ends with "USDT").
pub fn is_usdt_perpetual(symbol: &str) -> bool {
    symbol.ends_with("USDT")
}

/// Check if a symbol is a USDC perpetual (ends with "USDC").
pub fn is_usdc_perpetual(symbol: &str) -> bool {
    symbol.ends_with("USDC")
}

/// Check if a symbol is a USDC option (e.g., "BTC-31MAR23-28000-C").
///
/// Matches the pattern: uppercase letters, dash, anything, dash, P or C at end.
pub fn is_usdc_option(symbol: &str) -> bool {
    let parts: Vec<&str> = symbol.split('-').collect();
    if parts.len() < 3 {
        return false;
    }
    let last = *parts.last().unwrap();
    last == "P" || last == "C"
}

/// Return the opposite trading side: "Buy" ↔ "Sell".
pub fn opposite_side(side: &str) -> &'static str {
    match side {
        "Buy" => "Sell",
        "Sell" => "Buy",
        _ => panic!("Invalid side: expected \"Buy\" or \"Sell\", got \"{side}\""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamp_is_reasonable() {
        let ts = generate_timestamp();
        // Should be after 2024-01-01 in milliseconds
        assert!(ts > 1_704_067_200_000);
    }

    #[test]
    fn inverse_contracts() {
        assert!(is_inverse_contract("BTCUSD"));
        assert!(is_inverse_contract("ETHUSD"));
        assert!(is_inverse_contract("BTCUSDH24"));
        assert!(is_inverse_contract("ETHUSDM25"));
        assert!(is_inverse_contract("BTCUSDZ23"));

        assert!(!is_inverse_contract("BTCUSDT"));
        assert!(!is_inverse_contract("BTCUSDC"));
        assert!(!is_inverse_contract("BTC"));
    }

    #[test]
    fn usdt_perpetuals() {
        assert!(is_usdt_perpetual("BTCUSDT"));
        assert!(is_usdt_perpetual("ETHUSDT"));
        assert!(!is_usdt_perpetual("BTCUSD"));
        assert!(!is_usdt_perpetual("BTCUSDC"));
    }

    #[test]
    fn usdc_perpetuals() {
        assert!(is_usdc_perpetual("BTCUSDC"));
        assert!(!is_usdc_perpetual("BTCUSDT"));
    }

    #[test]
    fn usdc_options() {
        assert!(is_usdc_option("BTC-31MAR23-28000-C"));
        assert!(is_usdc_option("ETH-28APR23-1800-P"));
        assert!(!is_usdc_option("BTCUSDT"));
        assert!(!is_usdc_option("BTC-31MAR23"));
    }

    #[test]
    fn opposite_sides() {
        assert_eq!(opposite_side("Buy"), "Sell");
        assert_eq!(opposite_side("Sell"), "Buy");
    }

    #[test]
    #[should_panic(expected = "Invalid side")]
    fn opposite_side_invalid() {
        opposite_side("Hold");
    }
}
