//! High-level API methods organized by category.
//!
//! Each submodule implements endpoint methods on [`HttpManager`](crate::HttpManager)
//! using the typed parameter structs from [`params`](crate::params) and
//! returning the raw [`ApiResponse`](crate::ApiResponse) (JSON body).

pub mod account;
pub mod asset;
pub mod broker;
pub mod crypto_loan;
pub mod earn;
pub mod fiat;
pub mod institutional_loan;
pub mod market;
pub mod misc;
pub mod p2p;
pub mod position;
pub mod pre_upgrade;
pub mod rate_limit;
pub mod rfq;
pub mod spot_leverage;
pub mod spot_margin;
pub mod spread;
pub mod trade;
pub mod user;

use std::collections::HashMap;
use serde::Serialize;
use serde_json::Value;

/// Convert any `Serialize` struct into a `HashMap<String, Value>`.
///
/// This bridges the typed params structs to the `submit_request` interface
/// which expects `HashMap<String, Value>`.
pub(crate) fn params_to_map<T: Serialize>(params: &T) -> HashMap<String, Value> {
    let value = serde_json::to_value(params).unwrap_or(Value::Object(Default::default()));
    match value {
        Value::Object(map) => map.into_iter().collect(),
        _ => HashMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::market::GetKlineParams;

    #[test]
    fn test_params_to_map() {
        let params = GetKlineParams {
            category: "linear".into(),
            symbol: "BTCUSDT".into(),
            interval: "15".into(),
            start: Some(1000),
            end: None,
            limit: Some(200),
        };
        let map = params_to_map(&params);
        assert_eq!(map["category"], "linear");
        assert_eq!(map["symbol"], "BTCUSDT");
        assert_eq!(map["limit"], 200);
        assert!(!map.contains_key("end")); // None fields skipped
    }
}
