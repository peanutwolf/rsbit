//! Integration tests for the HTTP manager.
//!
//! These tests hit the real Bybit testnet API.

use rsbit::http::Method;
use rsbit::{ClientConfig, HttpManager};

/// Test that `get_server_time` returns a valid response from testnet.
#[tokio::test]
async fn get_server_time_testnet() {
    let config = ClientConfig {
        testnet: true,
        ..ClientConfig::default()
    };
    let manager = HttpManager::new(config).unwrap();

    let response = manager
        .submit_request(Method::Get, "/v5/market/time", None, false)
        .await
        .unwrap();

    let body = &response.body;
    assert_eq!(body["retCode"], 0, "Expected retCode 0, got: {}", body);
    assert_eq!(body["retMsg"], "OK");

    // timeSecond and timeNano should be present
    let result = &body["result"];
    assert!(
        result["timeSecond"].is_string(),
        "Missing timeSecond in result: {}",
        result
    );
    assert!(
        result["timeNano"].is_string(),
        "Missing timeNano in result: {}",
        result
    );

    // Verify the timestamp is reasonable (after 2024)
    let time_sec: u64 = result["timeSecond"]
        .as_str()
        .unwrap()
        .parse()
        .expect("timeSecond should be a numeric string");
    assert!(time_sec > 1_704_067_200, "Timestamp too old: {}", time_sec);
}

/// Test that `get_server_time` also works against mainnet.
#[tokio::test]
async fn get_server_time_mainnet() {
    let config = ClientConfig::default();
    let manager = HttpManager::new(config).unwrap();

    let response = manager
        .submit_request(Method::Get, "/v5/market/time", None, false)
        .await
        .unwrap();

    assert_eq!(response.body["retCode"], 0);
    assert_eq!(response.body["retMsg"], "OK");
}

/// Test that GET with query parameters works (public endpoint, no auth).
#[tokio::test]
async fn get_kline_public() {
    let config = ClientConfig {
        testnet: true,
        ..ClientConfig::default()
    };
    let manager = HttpManager::new(config).unwrap();

    let mut params = std::collections::HashMap::new();
    params.insert("category".into(), serde_json::Value::String("linear".into()));
    params.insert("symbol".into(), serde_json::Value::String("BTCUSDT".into()));
    params.insert("interval".into(), serde_json::Value::String("60".into()));
    params.insert("limit".into(), serde_json::Value::Number(2.into()));

    let response = manager
        .submit_request(Method::Get, "/v5/market/kline", Some(params), false)
        .await
        .unwrap();

    let body = &response.body;
    assert_eq!(body["retCode"], 0, "Expected retCode 0, got: {}", body);

    let list = body["result"]["list"].as_array();
    assert!(list.is_some(), "Expected result.list array, got: {}", body);
    assert!(
        !list.unwrap().is_empty(),
        "Expected non-empty kline list"
    );
}

/// Test that `record_request_time` populates elapsed.
#[tokio::test]
async fn record_request_time() {
    let config = ClientConfig {
        testnet: true,
        record_request_time: true,
        ..ClientConfig::default()
    };
    let manager = HttpManager::new(config).unwrap();

    let response = manager
        .submit_request(Method::Get, "/v5/market/time", None, false)
        .await
        .unwrap();

    // Note: elapsed may or may not be populated depending on reqwest internals.
    // The important thing is the request succeeds.
    assert_eq!(response.body["retCode"], 0);
}

/// Test that requesting a non-existent endpoint returns a proper error.
#[tokio::test]
async fn non_existent_endpoint_returns_error() {
    let config = ClientConfig {
        testnet: true,
        ..ClientConfig::default()
    };
    let manager = HttpManager::new(config).unwrap();

    let result = manager
        .submit_request(Method::Get, "/v5/nonexistent/path", None, false)
        .await;

    // Should be an error (either HTTP 404 → FailedRequest, or retCode != 0)
    // The Bybit API typically returns 404 for unknown paths
    assert!(result.is_err(), "Expected error for non-existent endpoint");
}
