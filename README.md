# rsbit

Rust SDK for the [Bybit](https://bybit.com) V5 API.

Provides async HTTP and WebSocket clients for trading on Bybit, with
HMAC-SHA256 and RSA authentication, automatic retry/rate-limit handling,
and full V5 API coverage.

Ported from the official Python SDK
[pybit](https://github.com/bybit-exchange/pybit)
to idiomatic Rust using [Claude Code](https://docs.anthropic.com/en/docs/claude-code)
by Anthropic. Currently tracking pybit v5.16.0.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rsbit = "0.2"
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `rustls-tls` | Yes | Use rustls for TLS (pure Rust, no system OpenSSL needed) |
| `native-tls` | No | Use the platform's native TLS (OpenSSL on Linux, Schannel on Windows) |

To use native TLS instead of rustls:

```toml
rsbit = { version = "0.2", default-features = false, features = ["native-tls"] }
```

## Quick Start

### HTTP Client

```rust
use rsbit::{ClientConfig, HTTP};
use rsbit::params::market::GetKlineParams;

#[tokio::main]
async fn main() -> rsbit::error::Result<()> {
    // Public (unauthenticated) client
    let client = HTTP::new(ClientConfig::default())?;

    // Server time
    let resp = client.get_server_time().await?;
    println!("{}", resp.body);

    // Kline data
    let kline = client.get_kline(GetKlineParams {
        category: "linear".into(),
        symbol: "BTCUSDT".into(),
        interval: "60".into(),
        limit: Some(5),
        ..Default::default()
    }).await?;
    println!("{}", kline.body);

    Ok(())
}
```

### WebSocket Streams

```rust
use std::sync::Arc;
use rsbit::ws::manager::{WsConfig, WsManager};

#[tokio::main]
async fn main() -> rsbit::error::Result<()> {
    let config = WsConfig::public("linear");
    let ws = WsManager::new(config);
    ws.connect().await?;

    // Orderbook with automatic delta processing
    ws.orderbook_stream(50, "BTCUSDT", Arc::new(|msg| {
        println!("Orderbook: {}", msg);
    })).await?;

    tokio::time::sleep(std::time::Duration::from_secs(30)).await;
    ws.close().await;
    Ok(())
}
```

### WebSocket Trading

```rust
use std::sync::Arc;
use rsbit::ws::manager::WsConfig;
use rsbit::ws::trading::WsTradingManager;

#[tokio::main]
async fn main() -> rsbit::error::Result<()> {
    let config = WsConfig::trade_testnet("api_key", "api_secret");
    let ws = WsTradingManager::new(config).with_recv_window(5000);
    ws.connect().await?;

    ws.place_order(
        serde_json::json!({
            "category": "linear",
            "symbol": "BTCUSDT",
            "side": "Buy",
            "orderType": "Limit",
            "qty": "0.001",
            "price": "20000",
        }),
        Arc::new(|response| {
            println!("Order response: {}", response);
        }),
    ).await?;

    ws.close().await;
    Ok(())
}
```

### P2P Trading

```rust
use rsbit::{ClientConfig, HTTP};
use rsbit::params::p2p::{GetOrdersParams, GetOnlineAdsParams};

#[tokio::main]
async fn main() -> rsbit::error::Result<()> {
    let client = HTTP::new(ClientConfig::mainnet("api_key", "api_secret"))?;

    // Browse online ads
    let ads = client.get_p2p_online_ads(GetOnlineAdsParams {
        token_id: "USDT".into(),
        currency_id: "USD".into(),
        side: "1".into(), // 1 = buy
    }).await?;
    println!("{}", ads.body);

    // List your orders
    let orders = client.get_p2p_orders(GetOrdersParams {
        page: "1".into(),
        size: "20".into(),
        ..Default::default()
    }).await?;
    println!("{}", orders.body);

    Ok(())
}
```

## Authentication

The SDK supports two authentication methods:

**HMAC-SHA256** (default):
```rust
let config = ClientConfig::testnet("api_key", "api_secret");
```

**RSA PKCS1v15**:
```rust
use rsbit::config::AuthMethod;

let config = ClientConfig::testnet("api_key", "rsa_private_key_pem")
    .with_auth_method(AuthMethod::Rsa);
```

## Configuration

```rust
use rsbit::ClientConfig;

// Testnet
let config = ClientConfig::testnet("key", "secret");

// Demo trading
let config = ClientConfig::testnet("key", "secret").with_demo();

// Custom TLD (e.g., Hong Kong)
use rsbit::config::TLD_HK;
let config = ClientConfig::mainnet("key", "secret").with_tld(TLD_HK);

// Custom retry settings
let config = ClientConfig::default()
    .with_max_retries(5)
    .with_retry_delay(std::time::Duration::from_secs(2));
```

## Examples

See the [`examples/`](examples/) directory:

- [`http_basic.rs`](examples/http_basic.rs) — HTTP client: server time, kline, orderbook
- [`ws_orderbook.rs`](examples/ws_orderbook.rs) — WebSocket orderbook and ticker streams
- [`ws_trading.rs`](examples/ws_trading.rs) — WebSocket order placement

Run an example:
```sh
cargo run --example http_basic
```

## API Coverage

The SDK covers all Bybit V5 API endpoints across 20 categories:

| Category | Description |
|----------|-------------|
| Market | Kline, tickers, orderbook, trades, instruments, funding rates |
| Trade | Create/amend/cancel orders, batch operations, borrow quota |
| Position | Position info, leverage, margin, risk limit, TP/SL |
| Account | Wallet balance, fee rates, account info, collateral, delta mode |
| Asset | Transfers, deposits, withdrawals, coin info, asset overview |
| User | API key management, account settings, agreements, referrals |
| P2P | Account info, ads, orders, counterparty, chat, payment types |
| Spot Margin | Toggle spot margin, set leverage, currency data |
| Spot Leverage | Leveraged token info and purchase/redeem |
| Broker | Broker earnings, sub-account management, rate limit management |
| Earn | Savings products, staking |
| Fiat | Fiat deposit/withdraw, quote, execute, balance |
| Crypto Loan | Collateral, loan, repay, max loan amount |
| Institutional Loan | Institutional lending endpoints |
| Pre-upgrade | Pre-upgrade order and trade history |
| Spread | Spread trading instruments, orderbook, tickers |
| Rate Limit | Rate limit status |
| RFQ | Request-for-quote trading |
| Misc | Announcements |

## License

MIT
