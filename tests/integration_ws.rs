//! Integration tests for the WebSocket manager.
//!
//! Connects to the live Bybit testnet WS, subscribes to a public topic,
//! and verifies that messages are received and dispatched correctly.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use rsbit::ws::manager::{WsConfig, WsManager};
use serde_json::Value;

#[tokio::test]
async fn connect_and_receive_tickers() {
    let config = WsConfig::public_testnet("linear");
    let ws = WsManager::new(config);

    ws.connect().await.unwrap();
    assert!(ws.is_connected());

    // Use an mpsc channel to collect messages from the callback
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Value>();
    let callback: rsbit::ws::subscription::Callback = Arc::new(move |msg| {
        let _ = tx.send(msg);
    });

    ws.subscribe("tickers.BTCUSDT", callback).await.unwrap();

    // Collect 3 messages (tickers come every ~100ms)
    let mut received = Vec::new();
    let timeout = tokio::time::sleep(Duration::from_secs(10));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            Some(msg) = rx.recv() => {
                received.push(msg);
                if received.len() >= 3 {
                    break;
                }
            }
            _ = &mut timeout => {
                panic!("Timed out waiting for 3 messages (got {})", received.len());
            }
        }
    }

    assert_eq!(received.len(), 3);
    for msg in &received {
        assert_eq!(
            msg.get("topic").and_then(|v| v.as_str()),
            Some("tickers.BTCUSDT")
        );
        assert!(msg.get("data").is_some());
    }

    ws.close().await;
    assert!(!ws.is_connected());
}

#[tokio::test]
async fn subscribe_multiple_topics() {
    let config = WsConfig::public_testnet("linear");
    let ws = WsManager::new(config);

    ws.connect().await.unwrap();

    let btc_count = Arc::new(AtomicU32::new(0));
    let eth_count = Arc::new(AtomicU32::new(0));

    let btc_counter = Arc::clone(&btc_count);
    let btc_cb: rsbit::ws::subscription::Callback = Arc::new(move |_| {
        btc_counter.fetch_add(1, Ordering::SeqCst);
    });

    let eth_counter = Arc::clone(&eth_count);
    let eth_cb: rsbit::ws::subscription::Callback = Arc::new(move |_| {
        eth_counter.fetch_add(1, Ordering::SeqCst);
    });

    ws.subscribe("tickers.BTCUSDT", btc_cb).await.unwrap();
    ws.subscribe("tickers.ETHUSDT", eth_cb).await.unwrap();

    // Wait for some messages
    tokio::time::sleep(Duration::from_secs(3)).await;

    assert!(btc_count.load(Ordering::SeqCst) > 0, "Expected BTC ticker messages");
    assert!(eth_count.load(Ordering::SeqCst) > 0, "Expected ETH ticker messages");

    ws.close().await;
}

#[tokio::test]
async fn unsubscribe_stops_messages() {
    let config = WsConfig::public_testnet("linear");
    let ws = WsManager::new(config);

    ws.connect().await.unwrap();

    let count = Arc::new(AtomicU32::new(0));
    let counter = Arc::clone(&count);
    let cb: rsbit::ws::subscription::Callback = Arc::new(move |_| {
        counter.fetch_add(1, Ordering::SeqCst);
    });

    ws.subscribe("tickers.BTCUSDT", cb).await.unwrap();

    // Wait for at least 1 message
    tokio::time::sleep(Duration::from_secs(2)).await;
    let before = count.load(Ordering::SeqCst);
    assert!(before > 0, "Expected some messages before unsubscribe");

    ws.unsubscribe("tickers.BTCUSDT").await.unwrap();

    // Wait a bit and check count didn't grow much (callback removed)
    tokio::time::sleep(Duration::from_secs(2)).await;
    let after = count.load(Ordering::SeqCst);
    // After unsubscribe, server should stop sending. Allow a small margin
    // for in-flight messages.
    assert!(
        after <= before + 2,
        "Expected no new messages after unsubscribe, got {} -> {}",
        before,
        after
    );

    ws.close().await;
}

#[tokio::test]
async fn connection_survives_beyond_ping_interval() {
    // Verify that the ping keepalive keeps the connection alive beyond 20s.
    // Use a short ping interval (5s) and wait 25s total to prove it works.
    let config = WsConfig::public_testnet("linear")
        .with_ping_interval(std::time::Duration::from_secs(5));
    let ws = WsManager::new(config);

    ws.connect().await.unwrap();
    assert!(ws.is_connected());

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Value>();
    let callback: rsbit::ws::subscription::Callback = Arc::new(move |msg| {
        let _ = tx.send(msg);
    });

    ws.subscribe("tickers.BTCUSDT", callback).await.unwrap();

    // Wait 25 seconds — this spans multiple ping intervals
    tokio::time::sleep(Duration::from_secs(25)).await;

    // Connection should still be alive
    assert!(ws.is_connected(), "Connection should survive beyond ping interval");

    // Should still be receiving messages
    let timeout = tokio::time::sleep(Duration::from_secs(5));
    tokio::pin!(timeout);

    // Drain any buffered messages and check we get a fresh one
    while rx.try_recv().is_ok() {}

    tokio::select! {
        Some(msg) = rx.recv() => {
            assert_eq!(
                msg.get("topic").and_then(|v| v.as_str()),
                Some("tickers.BTCUSDT")
            );
        }
        _ = &mut timeout => {
            panic!("No message received after 25s — connection may have died");
        }
    }

    ws.close().await;
}

#[tokio::test]
async fn reconnect_config_defaults() {
    let config = WsConfig::public_testnet("linear");
    assert_eq!(config.max_retries, 10);
    assert!(config.restart_on_error);
}

#[tokio::test]
async fn duplicate_subscribe_returns_error() {
    let config = WsConfig::public_testnet("linear");
    let ws = WsManager::new(config);

    ws.connect().await.unwrap();

    let cb: rsbit::ws::subscription::Callback = Arc::new(|_| {});
    ws.subscribe("tickers.BTCUSDT", cb).await.unwrap();

    let cb2: rsbit::ws::subscription::Callback = Arc::new(|_| {});
    let result = ws.subscribe("tickers.BTCUSDT", cb2).await;
    assert!(result.is_err(), "Duplicate subscribe should fail");

    ws.close().await;
}
