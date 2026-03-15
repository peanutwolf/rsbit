//! WebSocket trading — order operations over WebSocket.
//!
//! Mirrors Python's `_websocket_trading.py` and `WebSocketTrading` class.
//! Uses a request/response pattern with `reqId`-based correlation, unlike
//! the subscription-based `WsManager`.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::sync::{broadcast, Mutex, RwLock};
use tokio_tungstenite::tungstenite::Message;

use crate::auth;
use crate::error::{BybitError, Result};
use crate::helpers::generate_timestamp;
use crate::ws::manager::{is_pong, spawn_ping_task, WsConfig, WsWriter};
use crate::ws::subscription::Callback;
use crate::ws::ws_url;

/// WebSocket trading manager for order operations.
///
/// Connects to the Bybit `/v5/trade` WebSocket endpoint and provides
/// methods for placing, amending, and cancelling orders with async
/// callback-based response handling.
///
/// # Example
///
/// ```rust,no_run
/// use std::sync::Arc;
/// use rsbit::ws::manager::WsConfig;
/// use rsbit::ws::trading::WsTradingManager;
///
/// # async fn run() -> rsbit::error::Result<()> {
/// let config = WsConfig::trade_testnet("api_key", "api_secret");
/// let ws = WsTradingManager::new(config);
/// ws.connect().await?;
///
/// ws.place_order(
///     serde_json::json!({
///         "category": "linear",
///         "symbol": "BTCUSDT",
///         "side": "Buy",
///         "orderType": "Limit",
///         "qty": "0.001",
///         "price": "20000",
///     }),
///     Arc::new(|response| {
///         println!("Order response: {}", response);
///     }),
/// ).await?;
/// # Ok(())
/// # }
/// ```
pub struct WsTradingManager {
    config: Arc<WsConfig>,
    url: String,
    writer: WsWriter,
    connected: Arc<AtomicBool>,
    authenticated: Arc<AtomicBool>,
    /// Pending request callbacks, keyed by reqId. Popped on response.
    pending: Arc<RwLock<HashMap<String, Callback>>>,
    shutdown_tx: broadcast::Sender<()>,
    /// X-BAPI-RECV-WINDOW header value. 0 means omit.
    recv_window: u32,
    /// Referer header for affiliate tracking. Empty means omit.
    referral_id: String,
}

impl WsTradingManager {
    /// Create a new trading WebSocket manager. Does not connect yet.
    ///
    /// Use [`WsConfig::trade`] or [`WsConfig::trade_testnet`] for the config.
    pub fn new(config: WsConfig) -> Self {
        let url = ws_url(&config.client_config, &config.channel_type);
        let (shutdown_tx, _) = broadcast::channel(4);

        Self {
            config: Arc::new(config),
            url,
            writer: Arc::new(Mutex::new(None)),
            connected: Arc::new(AtomicBool::new(false)),
            authenticated: Arc::new(AtomicBool::new(false)),
            pending: Arc::new(RwLock::new(HashMap::new())),
            shutdown_tx,
            recv_window: 0,
            referral_id: String::new(),
        }
    }

    /// Set the recv window (X-BAPI-RECV-WINDOW header).
    pub fn with_recv_window(mut self, recv_window: u32) -> Self {
        self.recv_window = recv_window;
        self
    }

    /// Set the referral ID (Referer header).
    pub fn with_referral_id(mut self, referral_id: impl Into<String>) -> Self {
        self.referral_id = referral_id.into();
        self
    }

    /// Connect to the trading WebSocket server.
    ///
    /// Performs the TLS/WS handshake, authenticates, and starts the
    /// ping and read-loop tasks.
    pub async fn connect(&self) -> Result<()> {
        tracing::debug!("Connecting to {}", self.url);

        let (ws_stream, _response) = tokio_tungstenite::connect_async(&self.url)
            .await
            .map_err(BybitError::WebSocket)?;

        let (write, read) = ws_stream.split();

        // Store the writer
        {
            let mut w = self.writer.lock().await;
            *w = Some(write);
        }
        self.connected.store(true, Ordering::SeqCst);
        self.authenticated.store(false, Ordering::SeqCst);
        tracing::debug!("Connected to {}", self.url);

        // Authenticate (trading always requires auth)
        self.authenticate().await?;

        // Spawn the ping task
        spawn_ping_task(
            Arc::clone(&self.writer),
            self.config.ping_interval,
            Arc::clone(&self.connected),
            self.shutdown_tx.subscribe(),
        );

        // Spawn the read loop
        let read_pending = Arc::clone(&self.pending);
        let read_connected = Arc::clone(&self.connected);
        let read_authenticated = Arc::clone(&self.authenticated);
        let mut read_shutdown = self.shutdown_tx.subscribe();

        tokio::spawn(async move {
            let mut read = read;
            loop {
                tokio::select! {
                    msg = read.next() => {
                        match msg {
                            Some(Ok(Message::Text(text))) => {
                                dispatch_trading_message(
                                    &text,
                                    &read_pending,
                                    &read_authenticated,
                                ).await;
                            }
                            Some(Ok(Message::Close(_))) => {
                                tracing::debug!("Trading WS closed by server");
                                read_connected.store(false, Ordering::SeqCst);
                                break;
                            }
                            Some(Ok(_)) => {}
                            Some(Err(e)) => {
                                tracing::error!("Trading WS read error: {}", e);
                                read_connected.store(false, Ordering::SeqCst);
                                break;
                            }
                            None => {
                                tracing::debug!("Trading WS stream ended");
                                read_connected.store(false, Ordering::SeqCst);
                                break;
                            }
                        }
                    }
                    _ = read_shutdown.recv() => {
                        tracing::debug!("Trading read loop shutting down");
                        read_connected.store(false, Ordering::SeqCst);
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    /// Check if the trading WebSocket is connected.
    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    /// Gracefully close the trading WebSocket connection.
    pub async fn close(&self) {
        let _ = self.shutdown_tx.send(());

        {
            let mut w = self.writer.lock().await;
            if let Some(ref mut writer) = *w {
                let _ = writer.send(Message::Close(None)).await;
                *w = None;
            }
        }

        self.connected.store(false, Ordering::SeqCst);
        self.authenticated.store(false, Ordering::SeqCst);
        tracing::debug!("Trading WS closed");
    }

    // ── Order Operations ─────────────────────────────────────────────

    /// Place an order via WebSocket.
    ///
    /// Operation: `order.create`
    pub async fn place_order(&self, params: Value, callback: Callback) -> Result<()> {
        self.send_order_operation("order.create", params, callback)
            .await
    }

    /// Amend an existing order via WebSocket.
    ///
    /// Operation: `order.amend`
    pub async fn amend_order(&self, params: Value, callback: Callback) -> Result<()> {
        self.send_order_operation("order.amend", params, callback)
            .await
    }

    /// Cancel an order via WebSocket.
    ///
    /// Operation: `order.cancel`
    pub async fn cancel_order(&self, params: Value, callback: Callback) -> Result<()> {
        self.send_order_operation("order.cancel", params, callback)
            .await
    }

    /// Place multiple orders in a batch via WebSocket.
    ///
    /// Operation: `order.create-batch`
    pub async fn place_batch_order(&self, params: Value, callback: Callback) -> Result<()> {
        self.send_order_operation("order.create-batch", params, callback)
            .await
    }

    /// Amend multiple orders in a batch via WebSocket.
    ///
    /// Operation: `order.amend-batch`
    pub async fn amend_batch_order(&self, params: Value, callback: Callback) -> Result<()> {
        self.send_order_operation("order.amend-batch", params, callback)
            .await
    }

    /// Cancel multiple orders in a batch via WebSocket.
    ///
    /// Operation: `order.cancel-batch`
    pub async fn cancel_batch_order(&self, params: Value, callback: Callback) -> Result<()> {
        self.send_order_operation("order.cancel-batch", params, callback)
            .await
    }

    // ── Internal ─────────────────────────────────────────────────────

    /// Send a trading operation with a callback for the response.
    async fn send_order_operation(
        &self,
        operation: &str,
        params: Value,
        callback: Callback,
    ) -> Result<()> {
        let req_id = uuid::Uuid::new_v4().to_string();

        let mut header = serde_json::json!({
            "X-BAPI-TIMESTAMP": generate_timestamp(),
        });

        if self.recv_window > 0 {
            header["X-BAPI-RECV-WINDOW"] = serde_json::json!(self.recv_window);
        }
        if !self.referral_id.is_empty() {
            header["Referer"] = serde_json::json!(self.referral_id);
        }

        let message = serde_json::json!({
            "reqId": req_id,
            "header": header,
            "op": operation,
            "args": [params],
        });

        let msg_str = message.to_string();

        // Store callback before sending (so it's ready for the response)
        {
            let mut pending = self.pending.write().await;
            pending.insert(req_id, callback);
        }

        self.send_raw(&msg_str).await
    }

    /// Send the auth message.
    async fn authenticate(&self) -> Result<()> {
        let auth_msg = auth::build_ws_auth_message(
            &self.config.client_config,
            self.config.private_auth_expire,
        )?;

        tracing::debug!("Sending trading auth message");
        self.send_raw(&auth_msg).await?;

        // Wait for auth response (up to 5 seconds)
        for _ in 0..50 {
            if self.authenticated.load(Ordering::SeqCst) {
                tracing::debug!("Trading authentication successful");
                return Ok(());
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        Err(BybitError::WebSocket(
            tokio_tungstenite::tungstenite::Error::Protocol(
                tokio_tungstenite::tungstenite::error::ProtocolError::ResetWithoutClosingHandshake,
            ),
        ))
    }

    /// Send a raw text message through the WebSocket.
    async fn send_raw(&self, msg: &str) -> Result<()> {
        let mut w = self.writer.lock().await;
        match w.as_mut() {
            Some(writer) => {
                writer
                    .send(Message::Text(msg.into()))
                    .await
                    .map_err(BybitError::WebSocket)?;
                Ok(())
            }
            None => Err(BybitError::WebSocket(
                tokio_tungstenite::tungstenite::Error::AlreadyClosed,
            )),
        }
    }
}

impl std::fmt::Debug for WsTradingManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WsTradingManager")
            .field("url", &self.url)
            .field("connected", &self.connected.load(Ordering::SeqCst))
            .field("authenticated", &self.authenticated.load(Ordering::SeqCst))
            .finish()
    }
}

/// Dispatch a received trading WebSocket message.
///
/// Handles pong responses, auth confirmations, and trading operation
/// responses (matched by reqId).
async fn dispatch_trading_message(
    text: &str,
    pending: &Arc<RwLock<HashMap<String, Callback>>>,
    authenticated: &Arc<AtomicBool>,
) {
    let msg: Value = match serde_json::from_str(text) {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!("Failed to parse trading WS message: {} — raw: {}", e, text);
            return;
        }
    };

    // 1. Pong — swallow
    if is_pong(&msg) {
        return;
    }

    let op = msg.get("op").and_then(|v| v.as_str()).unwrap_or("");

    // 2. Auth response
    if op == "auth" {
        let ret_code = msg.get("retCode").and_then(|v| v.as_i64()).unwrap_or(-1);
        if ret_code == 0 {
            authenticated.store(true, Ordering::SeqCst);
            tracing::debug!("Trading auth confirmed");
        } else {
            tracing::error!("Trading auth failed: {}", msg);
        }
        return;
    }

    // 3. Trading response — match by reqId
    if let Some(req_id) = msg.get("reqId").and_then(|v| v.as_str()) {
        let ret_code = msg.get("retCode").and_then(|v| v.as_i64()).unwrap_or(-1);

        // Pop the callback
        let callback = {
            let mut p = pending.write().await;
            p.remove(req_id)
        };

        if ret_code != 0 {
            tracing::error!(
                "Trading request {} error: retCode={}, retMsg={}",
                req_id,
                ret_code,
                msg.get("retMsg").and_then(|v| v.as_str()).unwrap_or("unknown")
            );
        }

        if let Some(cb) = callback {
            cb(msg);
        } else {
            tracing::debug!("No pending callback for reqId: {}", req_id);
        }
        return;
    }

    // 4. Unknown
    tracing::debug!("Unhandled trading WS message: {}", msg);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ws::manager::WsConfig;

    #[test]
    fn ws_config_trade_testnet() {
        let config = WsConfig::trade_testnet("k", "s");
        assert!(config.is_private());
        assert_eq!(config.channel_type, "trade");
    }

    #[test]
    fn ws_config_trade_url() {
        let config = WsConfig::trade_testnet("k", "s");
        let ws = WsTradingManager::new(config);
        assert_eq!(ws.url, "wss://stream-testnet.bybit.com/v5/trade");
    }

    #[test]
    fn ws_config_trade_mainnet_url() {
        let config = WsConfig::trade("k", "s");
        let ws = WsTradingManager::new(config);
        assert_eq!(ws.url, "wss://stream.bybit.com/v5/trade");
    }

    #[test]
    fn trading_manager_new_does_not_connect() {
        let config = WsConfig::trade_testnet("k", "s");
        let ws = WsTradingManager::new(config);
        assert!(!ws.is_connected());
    }

    #[test]
    fn trading_message_format() {
        // Verify the JSON structure of a trading request message
        let req_id = "test-uuid";
        let ts = 1234567890000u64;

        let message = serde_json::json!({
            "reqId": req_id,
            "header": {
                "X-BAPI-TIMESTAMP": ts,
            },
            "op": "order.create",
            "args": [{
                "category": "linear",
                "symbol": "BTCUSDT",
                "side": "Buy",
                "orderType": "Limit",
                "qty": "0.001",
                "price": "20000",
            }],
        });

        assert_eq!(message["op"], "order.create");
        assert_eq!(message["reqId"], "test-uuid");
        assert!(message["args"].is_array());
        assert_eq!(message["args"][0]["symbol"], "BTCUSDT");
        assert_eq!(message["header"]["X-BAPI-TIMESTAMP"], ts);
    }

    #[test]
    fn trading_message_with_recv_window() {
        let mut header = serde_json::json!({
            "X-BAPI-TIMESTAMP": 1234567890000u64,
        });

        let recv_window = 5000u32;
        if recv_window > 0 {
            header["X-BAPI-RECV-WINDOW"] = serde_json::json!(recv_window);
        }

        assert_eq!(header["X-BAPI-RECV-WINDOW"], 5000);
    }

    #[test]
    fn with_recv_window_and_referral() {
        let config = WsConfig::trade_testnet("k", "s");
        let ws = WsTradingManager::new(config)
            .with_recv_window(5000)
            .with_referral_id("my-referral");

        assert_eq!(ws.recv_window, 5000);
        assert_eq!(ws.referral_id, "my-referral");
    }

    #[tokio::test]
    async fn dispatch_auth_success() {
        let pending = Arc::new(RwLock::new(HashMap::new()));
        let authenticated = Arc::new(AtomicBool::new(false));

        let msg = r#"{"op":"auth","retCode":0,"retMsg":"OK"}"#;
        dispatch_trading_message(msg, &pending, &authenticated).await;

        assert!(authenticated.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn dispatch_auth_failure() {
        let pending = Arc::new(RwLock::new(HashMap::new()));
        let authenticated = Arc::new(AtomicBool::new(false));

        let msg = r#"{"op":"auth","retCode":10003,"retMsg":"Invalid key"}"#;
        dispatch_trading_message(msg, &pending, &authenticated).await;

        assert!(!authenticated.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn dispatch_trading_response_invokes_callback() {
        use std::sync::atomic::AtomicU32;

        let pending = Arc::new(RwLock::new(HashMap::new()));
        let authenticated = Arc::new(AtomicBool::new(false));
        let count = Arc::new(AtomicU32::new(0));

        let counter = Arc::clone(&count);
        let cb: Callback = Arc::new(move |_| {
            counter.fetch_add(1, Ordering::SeqCst);
        });

        {
            let mut p = pending.write().await;
            p.insert("test-req-1".into(), cb);
        }

        let msg = r#"{"reqId":"test-req-1","retCode":0,"retMsg":"OK","data":{}}"#;
        dispatch_trading_message(msg, &pending, &authenticated).await;

        assert_eq!(count.load(Ordering::SeqCst), 1);
        // Callback should be removed
        assert!(pending.read().await.get("test-req-1").is_none());
    }

    #[tokio::test]
    async fn dispatch_error_response_invokes_callback() {
        use std::sync::atomic::AtomicU32;

        let pending = Arc::new(RwLock::new(HashMap::new()));
        let authenticated = Arc::new(AtomicBool::new(false));
        let count = Arc::new(AtomicU32::new(0));

        let counter = Arc::clone(&count);
        let cb: Callback = Arc::new(move |msg: Value| {
            // Callback receives the error response so it can handle it
            assert_eq!(msg["retCode"], 10001);
            counter.fetch_add(1, Ordering::SeqCst);
        });

        {
            let mut p = pending.write().await;
            p.insert("test-req-2".into(), cb);
        }

        let msg = r#"{"reqId":"test-req-2","retCode":10001,"retMsg":"Error"}"#;
        dispatch_trading_message(msg, &pending, &authenticated).await;

        assert_eq!(count.load(Ordering::SeqCst), 1);
        // Callback should be removed
        assert!(pending.read().await.get("test-req-2").is_none());
    }

    #[tokio::test]
    async fn dispatch_pong_swallowed() {
        let pending = Arc::new(RwLock::new(HashMap::new()));
        let authenticated = Arc::new(AtomicBool::new(false));

        let msg = r#"{"op":"pong","ret_msg":"pong"}"#;
        dispatch_trading_message(msg, &pending, &authenticated).await;
        // No crash, no side effects
    }
}
