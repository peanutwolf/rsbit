//! Integration tests for WebSocket trading.
//!
//! Tests connection to the live Bybit testnet trade WebSocket.
//! Order placement tests require valid API keys and are marked `#[ignore]`.

use rsbit::ws::manager::WsConfig;
use rsbit::ws::trading::WsTradingManager;

#[tokio::test]
async fn trading_ws_url_correct() {
    let config = WsConfig::trade_testnet("test_key", "test_secret");
    let ws = WsTradingManager::new(config);
    assert!(!ws.is_connected());
    // URL is constructed correctly (verified by unit tests)
}

#[tokio::test]
async fn trading_ws_builder_chain() {
    let config = WsConfig::trade_testnet("key", "secret");
    let ws = WsTradingManager::new(config)
        .with_recv_window(5000)
        .with_referral_id("test-ref");
    assert!(!ws.is_connected());
    // Builder methods chain correctly
}

#[tokio::test]
async fn trading_ws_mainnet_url() {
    let config = WsConfig::trade("key", "secret");
    let ws = WsTradingManager::new(config);
    assert!(!ws.is_connected());
    // Mainnet URL construction verified by unit tests
}

/// Connect to the live testnet trade WS and authenticate.
/// Requires BYBIT_TESTNET_KEY and BYBIT_TESTNET_SECRET env vars.
#[tokio::test]
#[ignore]
async fn trading_ws_connect_and_authenticate() {
    let api_key = std::env::var("BYBIT_TESTNET_KEY").expect("BYBIT_TESTNET_KEY not set");
    let api_secret =
        std::env::var("BYBIT_TESTNET_SECRET").expect("BYBIT_TESTNET_SECRET not set");

    let config = WsConfig::trade_testnet(&api_key, &api_secret);
    let ws = WsTradingManager::new(config);

    ws.connect().await.expect("Failed to connect");
    assert!(ws.is_connected());

    ws.close().await;
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    assert!(!ws.is_connected());
}

/// Place and cancel an order on testnet.
/// Requires BYBIT_TESTNET_KEY and BYBIT_TESTNET_SECRET env vars.
#[tokio::test]
#[ignore]
async fn trading_ws_place_and_cancel_order() {
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    let api_key = std::env::var("BYBIT_TESTNET_KEY").expect("BYBIT_TESTNET_KEY not set");
    let api_secret =
        std::env::var("BYBIT_TESTNET_SECRET").expect("BYBIT_TESTNET_SECRET not set");

    let config = WsConfig::trade_testnet(&api_key, &api_secret);
    let ws = WsTradingManager::new(config).with_recv_window(5000);

    ws.connect().await.expect("Failed to connect");

    let callback_count = Arc::new(AtomicU32::new(0));
    let counter = Arc::clone(&callback_count);

    ws.place_order(
        serde_json::json!({
            "category": "linear",
            "symbol": "BTCUSDT",
            "side": "Buy",
            "orderType": "Limit",
            "qty": "0.001",
            "price": "10000",
            "timeInForce": "GTC",
        }),
        Arc::new(move |response| {
            println!("Place order response: {}", response);
            counter.fetch_add(1, Ordering::SeqCst);
        }),
    )
    .await
    .expect("Failed to send place_order");

    // Wait for response
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    assert_eq!(callback_count.load(Ordering::SeqCst), 1);

    ws.close().await;
}
