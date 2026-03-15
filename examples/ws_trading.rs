//! WebSocket trading — place and cancel orders over WebSocket.
//!
//! Connects to the Bybit testnet trade WebSocket. Requires valid testnet
//! API keys set via environment variables:
//!
//! ```sh
//! BYBIT_TESTNET_KEY=... BYBIT_TESTNET_SECRET=... cargo run --example ws_trading
//! ```

use std::sync::Arc;

use rsbit::ws::manager::WsConfig;
use rsbit::ws::trading::WsTradingManager;

#[tokio::main]
async fn main() -> rsbit::error::Result<()> {
    tracing_subscriber::fmt::init();

    let api_key = std::env::var("BYBIT_TESTNET_KEY")
        .expect("Set BYBIT_TESTNET_KEY env var");
    let api_secret = std::env::var("BYBIT_TESTNET_SECRET")
        .expect("Set BYBIT_TESTNET_SECRET env var");

    // Connect to testnet trade WebSocket
    let config = WsConfig::trade_testnet(&api_key, &api_secret);
    let ws = WsTradingManager::new(config).with_recv_window(5000);

    ws.connect().await?;
    println!("Connected and authenticated to Bybit testnet trade WS");

    // Place a limit order (far from market to avoid fills)
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
        Arc::new(|response| {
            let ret_code = response["retCode"].as_i64().unwrap_or(-1);
            if ret_code == 0 {
                let order_id = &response["data"]["orderId"];
                println!("Order placed: orderId={}", order_id);
            } else {
                println!(
                    "Order failed: retCode={}, retMsg={}",
                    ret_code, response["retMsg"]
                );
            }
        }),
    )
    .await?;

    // Wait for response
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    ws.close().await;
    println!("Closed.");
    Ok(())
}
