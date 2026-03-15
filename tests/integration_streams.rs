//! Integration tests for stream subscription methods.
//!
//! Connects to the live Bybit testnet WS, subscribes using the convenience
//! stream methods, and verifies delta processing works correctly.

use std::sync::Arc;
use std::time::Duration;

use rsbit::ws::manager::{WsConfig, WsManager};
use serde_json::Value;

#[tokio::test]
async fn orderbook_stream_receives_snapshots() {
    let config = WsConfig::public_testnet("linear");
    let ws = WsManager::new(config);
    ws.connect().await.unwrap();

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Value>();
    let callback: rsbit::ws::subscription::Callback = Arc::new(move |msg| {
        let _ = tx.send(msg);
    });

    ws.orderbook_stream(50, "BTCUSDT", callback).await.unwrap();

    // Collect at least 3 messages to verify delta processing
    let mut received = Vec::new();
    let timeout = tokio::time::sleep(Duration::from_secs(15));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            Some(msg) = rx.recv() => {
                // Every message should be type "snapshot" (delta processor converts)
                assert_eq!(
                    msg.get("type").and_then(|v| v.as_str()),
                    Some("snapshot"),
                    "Expected snapshot type, got: {:?}", msg.get("type")
                );
                // Should have full orderbook data with bids and asks
                let data = msg.get("data").expect("Missing data field");
                assert!(data.get("b").is_some(), "Missing bids");
                assert!(data.get("a").is_some(), "Missing asks");

                received.push(msg);
                if received.len() >= 3 {
                    break;
                }
            }
            _ = &mut timeout => {
                panic!("Timed out waiting for 3 orderbook messages (got {})", received.len());
            }
        }
    }

    // After snapshot + deltas, bids/asks should be non-empty
    let last = received.last().unwrap();
    let bids = last["data"]["b"].as_array().unwrap();
    let asks = last["data"]["a"].as_array().unwrap();
    assert!(!bids.is_empty(), "Expected non-empty bids");
    assert!(!asks.is_empty(), "Expected non-empty asks");

    ws.close().await;
}

#[tokio::test]
async fn ticker_stream_receives_snapshots() {
    let config = WsConfig::public_testnet("linear");
    let ws = WsManager::new(config);
    ws.connect().await.unwrap();

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Value>();
    let callback: rsbit::ws::subscription::Callback = Arc::new(move |msg| {
        let _ = tx.send(msg);
    });

    ws.ticker_stream("BTCUSDT", callback).await.unwrap();

    let mut received = Vec::new();
    let timeout = tokio::time::sleep(Duration::from_secs(10));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            Some(msg) = rx.recv() => {
                assert_eq!(
                    msg.get("type").and_then(|v| v.as_str()),
                    Some("snapshot"),
                    "Expected snapshot type"
                );
                received.push(msg);
                if received.len() >= 3 {
                    break;
                }
            }
            _ = &mut timeout => {
                panic!("Timed out waiting for 3 ticker messages (got {})", received.len());
            }
        }
    }

    ws.close().await;
}

#[tokio::test]
async fn trade_stream_basic() {
    let config = WsConfig::public_testnet("linear");
    let ws = WsManager::new(config);
    ws.connect().await.unwrap();

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Value>();
    let callback: rsbit::ws::subscription::Callback = Arc::new(move |msg| {
        let _ = tx.send(msg);
    });

    ws.trade_stream("BTCUSDT", callback).await.unwrap();

    let timeout = tokio::time::sleep(Duration::from_secs(15));
    tokio::pin!(timeout);

    tokio::select! {
        Some(msg) = rx.recv() => {
            assert_eq!(
                msg.get("topic").and_then(|v| v.as_str()),
                Some("publicTrade.BTCUSDT")
            );
            assert!(msg.get("data").is_some());
        }
        _ = &mut timeout => {
            panic!("Timed out waiting for trade message");
        }
    }

    ws.close().await;
}

#[tokio::test]
async fn kline_stream_basic() {
    let config = WsConfig::public_testnet("linear");
    let ws = WsManager::new(config);
    ws.connect().await.unwrap();

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Value>();
    let callback: rsbit::ws::subscription::Callback = Arc::new(move |msg| {
        let _ = tx.send(msg);
    });

    ws.kline_stream("1", "BTCUSDT", callback).await.unwrap();

    let timeout = tokio::time::sleep(Duration::from_secs(15));
    tokio::pin!(timeout);

    tokio::select! {
        Some(msg) = rx.recv() => {
            assert_eq!(
                msg.get("topic").and_then(|v| v.as_str()),
                Some("kline.1.BTCUSDT")
            );
        }
        _ = &mut timeout => {
            panic!("Timed out waiting for kline message");
        }
    }

    ws.close().await;
}
