//! WebSocket orderbook stream — subscribe and print snapshots.
//!
//! Connects to the Bybit testnet public WebSocket and subscribes to the
//! BTCUSDT orderbook with depth 50. Delta processing is handled automatically;
//! the callback always receives a full snapshot.
//!
//! Run with:
//! ```sh
//! cargo run --example ws_orderbook
//! ```

use std::sync::Arc;

use rsbit::ws::manager::{WsConfig, WsManager};

#[tokio::main]
async fn main() -> rsbit::error::Result<()> {
    tracing_subscriber::fmt::init();

    // Connect to testnet public linear channel
    let config = WsConfig::public_testnet("linear");
    let ws = WsManager::new(config);
    ws.connect().await?;

    println!("Connected to Bybit testnet WebSocket");

    // Subscribe to orderbook with depth 50
    ws.orderbook_stream(
        50,
        "BTCUSDT",
        Arc::new(|msg| {
            let data = &msg["data"];
            let bid_count = data["b"].as_array().map(|a| a.len()).unwrap_or(0);
            let ask_count = data["a"].as_array().map(|a| a.len()).unwrap_or(0);
            println!(
                "Orderbook snapshot — {} bids, {} asks, seq={}",
                bid_count,
                ask_count,
                data["seq"]
            );
        }),
    )
    .await?;

    // Subscribe to ticker (delta-processed)
    ws.ticker_stream(
        "BTCUSDT",
        Arc::new(|msg| {
            let data = &msg["data"];
            println!(
                "Ticker — lastPrice={}, volume24h={}",
                data["lastPrice"], data["volume24h"]
            );
        }),
    )
    .await?;

    println!("Subscribed to orderbook.50.BTCUSDT and tickers.BTCUSDT");
    println!("Listening for 30 seconds...\n");

    tokio::time::sleep(std::time::Duration::from_secs(30)).await;

    ws.close().await;
    println!("\nClosed.");
    Ok(())
}
