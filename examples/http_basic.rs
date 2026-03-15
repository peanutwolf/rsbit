//! Basic HTTP client usage — server time, kline, and orderbook queries.
//!
//! Run with:
//! ```sh
//! cargo run --example http_basic
//! ```

use rsbit::params::market::{GetKlineParams, GetOrderbookParams, GetTickersParams};
use rsbit::{ClientConfig, HTTP};

#[tokio::main]
async fn main() -> rsbit::error::Result<()> {
    tracing_subscriber::fmt::init();

    // ── Public (unauthenticated) client ──────────────────────────────
    let client = HTTP::new(ClientConfig::default())?;

    // 1. Server time
    let resp = client.get_server_time().await?;
    println!("Server time: {}", resp.body);

    // 2. Kline data (BTCUSDT 1-hour candles)
    let kline = client
        .get_kline(GetKlineParams {
            category: "linear".into(),
            symbol: "BTCUSDT".into(),
            interval: "60".into(),
            limit: Some(5),
            ..Default::default()
        })
        .await?;
    println!("\nKline (last 5 candles):\n{}", kline.body);

    // 3. Orderbook
    let book = client
        .get_orderbook(GetOrderbookParams {
            category: "linear".into(),
            symbol: "BTCUSDT".into(),
            limit: Some(5),
        })
        .await?;
    println!("\nOrderbook (top 5):\n{}", book.body);

    // 4. Tickers
    let tickers = client
        .get_tickers(GetTickersParams {
            category: "linear".into(),
            symbol: Some("BTCUSDT".into()),
            ..Default::default()
        })
        .await?;
    println!("\nTicker:\n{}", tickers.body);

    // ── Authenticated client (testnet) ───────────────────────────────
    // Uncomment and set your keys to test authenticated endpoints:
    //
    // let auth_client = HTTP::new(ClientConfig::testnet("YOUR_KEY", "YOUR_SECRET"))?;
    // let wallet = auth_client.get_wallet_balance(GetWalletBalanceParams {
    //     account_type: "UNIFIED".into(),
    //     ..Default::default()
    // }).await?;
    // println!("\nWallet balance: {}", wallet.body);

    Ok(())
}
