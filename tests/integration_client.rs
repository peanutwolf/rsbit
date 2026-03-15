//! Integration tests for the unified `HTTP` client.
//!
//! Validates that `HTTP` exposes methods from multiple API categories
//! via `Deref<Target = HttpManager>`.

use rsbit::response::market::*;
use rsbit::{ClientConfig, HTTP};

fn mainnet_client() -> HTTP {
    HTTP::new(ClientConfig::default()).unwrap()
}

#[tokio::test]
async fn http_get_server_time() {
    let client = mainnet_client();
    let resp = client.get_server_time().await.unwrap();
    let typed: ServerTimeResponse = serde_json::from_value(resp.body).unwrap();
    assert!(typed.is_ok());
}

#[tokio::test]
async fn http_get_kline() {
    let client = mainnet_client();
    let resp = client
        .get_kline(rsbit::params::market::GetKlineParams {
            category: "linear".into(),
            symbol: "BTCUSDT".into(),
            interval: "60".into(),
            limit: Some(5),
            ..Default::default()
        })
        .await
        .unwrap();
    let typed: KlineResponse = serde_json::from_value(resp.body).unwrap();
    assert!(typed.is_ok());
    assert!(!typed.result.list.is_empty());
}

#[tokio::test]
async fn http_get_announcement() {
    let client = mainnet_client();
    let resp = client
        .get_announcement(rsbit::params::misc::GetAnnouncementParams {
            locale: "en-US".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(resp.body["retCode"], 0);
}

#[tokio::test]
async fn http_spread_get_instruments_info() {
    let client = mainnet_client();
    let resp = client
        .spread_get_instruments_info(Default::default())
        .await
        .unwrap();
    assert_eq!(resp.body["retCode"], 0);
}

#[tokio::test]
async fn http_config_accessors() {
    let client = mainnet_client();
    assert_eq!(client.endpoint(), "https://api.bybit.com");
    assert!(!client.config().testnet);
}
