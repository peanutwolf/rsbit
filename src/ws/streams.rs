//! Convenience stream subscription methods for the Bybit V5 WebSocket API.
//!
//! These are thin wrappers around [`WsManager::subscribe()`] that construct
//! the correct topic string for each stream type. Mirrors Python's
//! `unified_trading.py` stream methods.

use crate::error::Result;
use crate::ws::delta::{wrap_orderbook_callback, wrap_ticker_callback};
use crate::ws::manager::WsManager;
use crate::ws::subscription::Callback;

impl WsManager {
    // ── Public Market Data Streams ───────────────────────────────────

    /// Subscribe to the orderbook stream with delta processing.
    ///
    /// The callback always receives a full snapshot, even when the server
    /// sends incremental deltas. Local state is maintained automatically.
    ///
    /// Topic: `orderbook.{depth}.{symbol}`
    pub async fn orderbook_stream(
        &self,
        depth: u32,
        symbol: &str,
        callback: Callback,
    ) -> Result<()> {
        let topic = format!("orderbook.{}.{}", depth, symbol);
        self.subscribe(topic, wrap_orderbook_callback(callback)).await
    }

    /// Subscribe to the RPI orderbook stream with delta processing.
    ///
    /// Topic: `orderbook.rpi.{symbol}`
    pub async fn rpi_orderbook_stream(
        &self,
        symbol: &str,
        callback: Callback,
    ) -> Result<()> {
        let topic = format!("orderbook.rpi.{}", symbol);
        self.subscribe(topic, wrap_orderbook_callback(callback))
            .await
    }

    /// Subscribe to the ticker stream with delta processing.
    ///
    /// The callback always receives a full snapshot with all fields,
    /// even when the server sends partial delta updates.
    ///
    /// Topic: `tickers.{symbol}`
    pub async fn ticker_stream(
        &self,
        symbol: &str,
        callback: Callback,
    ) -> Result<()> {
        let topic = format!("tickers.{}", symbol);
        self.subscribe(topic, wrap_ticker_callback(callback)).await
    }

    /// Subscribe to the public trade stream.
    ///
    /// Topic: `publicTrade.{symbol}`
    pub async fn trade_stream(
        &self,
        symbol: &str,
        callback: Callback,
    ) -> Result<()> {
        let topic = format!("publicTrade.{}", symbol);
        self.subscribe(topic, callback).await
    }

    /// Subscribe to the kline (candlestick) stream.
    ///
    /// Topic: `kline.{interval}.{symbol}`
    pub async fn kline_stream(
        &self,
        interval: &str,
        symbol: &str,
        callback: Callback,
    ) -> Result<()> {
        let topic = format!("kline.{}.{}", interval, symbol);
        self.subscribe(topic, callback).await
    }

    /// Subscribe to the liquidation stream.
    ///
    /// Topic: `liquidation.{symbol}`
    #[deprecated(note = "Use all_liquidation_stream() instead")]
    pub async fn liquidation_stream(
        &self,
        symbol: &str,
        callback: Callback,
    ) -> Result<()> {
        let topic = format!("liquidation.{}", symbol);
        self.subscribe(topic, callback).await
    }

    /// Subscribe to the all-liquidation stream.
    ///
    /// Topic: `allLiquidation.{symbol}`
    pub async fn all_liquidation_stream(
        &self,
        symbol: &str,
        callback: Callback,
    ) -> Result<()> {
        let topic = format!("allLiquidation.{}", symbol);
        self.subscribe(topic, callback).await
    }

    /// Subscribe to the leveraged token ticker stream with delta processing.
    ///
    /// Topic: `tickers_lt.{symbol}`
    pub async fn lt_ticker_stream(
        &self,
        symbol: &str,
        callback: Callback,
    ) -> Result<()> {
        let topic = format!("tickers_lt.{}", symbol);
        self.subscribe(topic, wrap_ticker_callback(callback)).await
    }

    /// Subscribe to the leveraged token kline stream.
    ///
    /// Topic: `kline_lt.{interval}.{symbol}`
    pub async fn lt_kline_stream(
        &self,
        interval: &str,
        symbol: &str,
        callback: Callback,
    ) -> Result<()> {
        let topic = format!("kline_lt.{}.{}", interval, symbol);
        self.subscribe(topic, callback).await
    }

    /// Subscribe to the leveraged token NAV stream.
    ///
    /// Topic: `lt.{symbol}`
    pub async fn lt_nav_stream(
        &self,
        symbol: &str,
        callback: Callback,
    ) -> Result<()> {
        let topic = format!("lt.{}", symbol);
        self.subscribe(topic, callback).await
    }

    /// Subscribe to the insurance pool stream.
    ///
    /// Topic: `insurance.{contract_group}`
    pub async fn insurance_pool_stream(
        &self,
        contract_group: &str,
        callback: Callback,
    ) -> Result<()> {
        let topic = format!("insurance.{}", contract_group);
        self.subscribe(topic, callback).await
    }

    /// Subscribe to the price limit stream.
    ///
    /// Topic: `priceLimit.{symbol}`
    pub async fn price_limit_stream(
        &self,
        symbol: &str,
        callback: Callback,
    ) -> Result<()> {
        let topic = format!("priceLimit.{}", symbol);
        self.subscribe(topic, callback).await
    }

    // ── Private Streams ──────────────────────────────────────────────

    /// Subscribe to the position stream (private).
    ///
    /// Topic: `position`
    pub async fn position_stream(&self, callback: Callback) -> Result<()> {
        self.subscribe("position", callback).await
    }

    /// Subscribe to the order stream (private).
    ///
    /// Topic: `order`
    pub async fn order_stream(&self, callback: Callback) -> Result<()> {
        self.subscribe("order", callback).await
    }

    /// Subscribe to the execution stream (private).
    ///
    /// Topic: `execution`
    pub async fn execution_stream(&self, callback: Callback) -> Result<()> {
        self.subscribe("execution", callback).await
    }

    /// Subscribe to the fast execution stream (private).
    ///
    /// Topic: `execution.fast`
    pub async fn fast_execution_stream(&self, callback: Callback) -> Result<()> {
        self.subscribe("execution.fast", callback).await
    }

    /// Subscribe to the wallet stream (private).
    ///
    /// Topic: `wallet`
    pub async fn wallet_stream(&self, callback: Callback) -> Result<()> {
        self.subscribe("wallet", callback).await
    }

    /// Subscribe to the greeks stream (private, options only).
    ///
    /// Topic: `greeks`
    pub async fn greek_stream(&self, callback: Callback) -> Result<()> {
        self.subscribe("greeks", callback).await
    }
}
