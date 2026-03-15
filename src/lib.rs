//! # rsbit
//!
//! Rust SDK for the [Bybit](https://bybit.com) V5 API.
//!
//! Provides async HTTP and WebSocket clients for trading on Bybit,
//! with HMAC-SHA256 and RSA authentication, automatic retry/rate-limit
//! handling, and full V5 API coverage.
//!
//! ## Features
//!
//! - **HTTP client** ([`HTTP`] / [`HttpManager`]) — all V5 REST endpoints
//!   across 19 categories (market, trade, position, account, asset, etc.)
//! - **WebSocket streams** ([`ws::WsManager`]) — public and private data
//!   streams with automatic orderbook/ticker delta processing
//! - **WebSocket trading** ([`ws::WsTradingManager`]) — place, amend, and
//!   cancel orders over WebSocket with `reqId`-based correlation
//! - **Authentication** — HMAC-SHA256 (default) and RSA PKCS1v15
//! - **Retry & rate-limit** — automatic retry with configurable backoff
//!
//! ## Quick Start — HTTP
//!
//! ```rust,no_run
//! use rsbit::{ClientConfig, HTTP};
//! use rsbit::params::market::GetKlineParams;
//!
//! #[tokio::main]
//! async fn main() -> rsbit::error::Result<()> {
//!     let client = HTTP::new(ClientConfig::default())?;
//!
//!     // Public endpoint — no API keys needed
//!     let resp = client.get_server_time().await?;
//!     println!("Server time: {}", resp.body);
//!
//!     // Market data
//!     let kline = client.get_kline(GetKlineParams {
//!         category: "linear".into(),
//!         symbol: "BTCUSDT".into(),
//!         interval: "60".into(),
//!         limit: Some(5),
//!         ..Default::default()
//!     }).await?;
//!     println!("Kline: {}", kline.body);
//!     Ok(())
//! }
//! ```
//!
//! ## Quick Start — WebSocket
//!
//! ```rust,no_run
//! use std::sync::Arc;
//! use rsbit::ws::manager::{WsConfig, WsManager};
//!
//! # async fn run() -> rsbit::error::Result<()> {
//! let ws = WsManager::new(WsConfig::public("linear"));
//! ws.connect().await?;
//!
//! // Orderbook with automatic delta → snapshot processing
//! ws.orderbook_stream(50, "BTCUSDT", Arc::new(|msg| {
//!     println!("{}", msg);
//! })).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Modules
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`config`] | Client configuration (testnet, auth, retry, TLD) |
//! | [`http`] | HTTP client and request handling |
//! | [`ws`] | WebSocket streams and trading |
//! | [`api`] | Endpoint method implementations |
//! | [`params`] | Typed request parameter structs |
//! | [`response`] | Typed response structs |
//! | [`auth`] | HMAC and RSA signing |
//! | [`error`] | Error types |

pub mod api;
pub mod auth;
pub mod config;
pub mod endpoints;
pub mod error;
pub mod helpers;
pub mod http;
pub mod params;
pub mod response;
pub mod ws;

pub use config::ClientConfig;
pub use error::BybitError;
pub use http::{HttpManager, ApiResponse};
pub use http::client::HTTP;
