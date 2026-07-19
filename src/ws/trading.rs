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

/// Hook invoked after each successful reconnect + re-auth.
type ReconnectHook = Arc<dyn Fn() + Send + Sync>;

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
    /// Invoked after each successful reconnect + re-auth.
    on_reconnect: Arc<std::sync::RwLock<Option<ReconnectHook>>>,
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
            on_reconnect: Arc::new(std::sync::RwLock::new(None)),
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

    /// Register a hook invoked after each successful reconnect + re-auth,
    /// e.g. to reconcile state that requests lost on the old connection
    /// may have left behind.
    pub fn set_on_reconnect(&self, hook: impl Fn() + Send + Sync + 'static) {
        *self.on_reconnect.write().unwrap() = Some(Arc::new(hook));
    }

    /// Connect to the trading WebSocket server.
    ///
    /// Performs the TLS/WS handshake, authenticates, and starts the
    /// ping and read-loop tasks. The read loop automatically reconnects and
    /// re-authenticates on connection loss when `restart_on_error` is enabled
    /// in the config.
    pub async fn connect(&self) -> Result<()> {
        let read = self.establish_connection().await?;

        // Spawn the self-healing read loop BEFORE authenticating so that auth
        // responses are processed (mirrors pybit's approach where run_forever()
        // starts the read thread before _auth() is called).
        let read_pending = Arc::clone(&self.pending);
        let read_connected = Arc::clone(&self.connected);
        let read_authenticated = Arc::clone(&self.authenticated);
        let read_writer = Arc::clone(&self.writer);
        let read_shutdown_tx = self.shutdown_tx.clone();
        let mut read_shutdown = self.shutdown_tx.subscribe();
        let read_on_reconnect = Arc::clone(&self.on_reconnect);
        let config = Arc::clone(&self.config);
        let url = self.url.clone();

        tokio::spawn(async move {
            run_trading_read_loop(
                read,
                &mut read_shutdown,
                &read_pending,
                &read_connected,
                &read_authenticated,
                &read_writer,
                &read_shutdown_tx,
                &read_on_reconnect,
                &config,
                &url,
            )
            .await;
        });

        // Authenticate (trading always requires auth)
        self.authenticate().await?;

        // Spawn the ping task
        spawn_ping_task(
            Arc::clone(&self.writer),
            self.config.ping_interval,
            Arc::clone(&self.connected),
            self.shutdown_tx.subscribe(),
        );

        Ok(())
    }

    /// Perform the WebSocket handshake and store the write half.
    ///
    /// Returns the read half so the caller can drive the read loop.
    async fn establish_connection(&self) -> Result<RawReader> {
        tracing::debug!("Connecting to {}", self.url);

        let (ws_stream, _response) =
            tokio_tungstenite::connect_async_with_config(&self.url, None, true)
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

        Ok(read)
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
        let req_id = super::next_req_id();
        let timestamp = generate_timestamp();
        let params_json = params.to_string();

        let header = if self.recv_window > 0 && !self.referral_id.is_empty() {
            format!(
                r#"{{"X-BAPI-TIMESTAMP":{},"X-BAPI-RECV-WINDOW":{},"Referer":"{}"}}"#,
                timestamp, self.recv_window, self.referral_id
            )
        } else if self.recv_window > 0 {
            format!(
                r#"{{"X-BAPI-TIMESTAMP":{},"X-BAPI-RECV-WINDOW":{}}}"#,
                timestamp, self.recv_window
            )
        } else if !self.referral_id.is_empty() {
            format!(
                r#"{{"X-BAPI-TIMESTAMP":{},"Referer":"{}"}}"#,
                timestamp, self.referral_id
            )
        } else {
            format!(r#"{{"X-BAPI-TIMESTAMP":{}}}"#, timestamp)
        };

        let msg_str = format!(
            r#"{{"reqId":"{}","header":{},"op":"{}","args":[{}]}}"#,
            req_id, header, operation, params_json
        );

        // Send first to minimize latency — the network RTT ensures the
        // callback is registered well before the response arrives.
        self.send_raw(&msg_str).await?;

        {
            let mut pending = self.pending.write().await;
            pending.insert(req_id, callback);
        }

        Ok(())
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

/// The raw WebSocket read-half type.
type RawReader = futures_util::stream::SplitStream<
    tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
>;

/// Run the trading read loop with automatic reconnection on disconnect.
///
/// When the connection drops and `restart_on_error` is enabled, this function
/// reconnects up to `max_retries` times (0 = infinite), re-authenticating on
/// each successful reconnection so subsequent order operations succeed instead
/// of failing forever against a dead socket.
#[allow(clippy::too_many_arguments)]
async fn run_trading_read_loop(
    mut read: RawReader,
    shutdown: &mut broadcast::Receiver<()>,
    pending: &Arc<RwLock<HashMap<String, Callback>>>,
    connected: &Arc<AtomicBool>,
    authenticated: &Arc<AtomicBool>,
    writer: &WsWriter,
    shutdown_tx: &broadcast::Sender<()>,
    on_reconnect: &Arc<std::sync::RwLock<Option<ReconnectHook>>>,
    config: &Arc<WsConfig>,
    url: &str,
) {
    let mut retry_count: u32 = 0;

    loop {
        // Inner read loop — processes messages until disconnect.
        let should_reconnect =
            read_until_disconnect(&mut read, shutdown, pending, connected, authenticated).await;

        // Drop the stale writer so any in-flight `send_raw` fails fast with
        // `AlreadyClosed` instead of writing into a broken socket.
        {
            let mut w = writer.lock().await;
            *w = None;
        }

        // Responses for requests in flight at disconnect can never arrive:
        // fail their callbacks so callers roll back optimistic state (e.g.
        // local order placeholders) instead of waiting forever.
        fail_pending_requests(pending).await;

        if !should_reconnect || !config.restart_on_error {
            break;
        }

        retry_count += 1;
        if config.max_retries > 0 && retry_count > config.max_retries {
            tracing::error!(
                "Trading WS: max reconnect attempts ({}) exceeded",
                config.max_retries
            );
            break;
        }

        // Exponential backoff: 2s, 4s, 8s, 16s, ... capped at 30s.
        let delay = Duration::from_secs((1u64 << retry_count.min(4)).min(30));
        tracing::info!(
            "Trading WS reconnecting in {:?} (attempt {}{})",
            delay,
            retry_count,
            if config.max_retries > 0 {
                format!("/{}", config.max_retries)
            } else {
                String::new()
            }
        );

        tokio::select! {
            _ = tokio::time::sleep(delay) => {}
            _ = shutdown.recv() => {
                tracing::debug!("Trading reconnect cancelled by shutdown");
                break;
            }
        }

        // Attempt reconnection.
        match reconnect_trading(url, writer, connected, authenticated, pending, config).await {
            Ok(new_read) => {
                read = new_read;
                retry_count = 0; // Reset on successful reconnect
                tracing::info!("Trading WS reconnected successfully");

                // Spawn a fresh ping task for this new connection.
                spawn_ping_task(
                    Arc::clone(writer),
                    config.ping_interval,
                    Arc::clone(connected),
                    shutdown_tx.subscribe(),
                );

                let hook = on_reconnect.read().unwrap().clone();
                if let Some(hook) = hook {
                    hook();
                }
            }
            Err(e) => {
                tracing::error!("Trading WS reconnect failed: {}", e);
                // Loop continues — will retry after backoff.
            }
        }
    }
}

/// Fail every pending request callback with a synthetic error response.
///
/// Called on disconnect: responses for requests that were in flight can never
/// arrive on the dead connection, and callers rely on the error path to roll
/// back optimistic state (e.g. local order placeholders).
async fn fail_pending_requests(pending: &Arc<RwLock<HashMap<String, Callback>>>) {
    let drained: Vec<(String, Callback)> = pending.write().await.drain().collect();
    if drained.is_empty() {
        return;
    }
    tracing::warn!(
        "Trading WS disconnected with {} request(s) in flight — failing their callbacks",
        drained.len()
    );
    for (req_id, cb) in drained {
        cb(serde_json::json!({
            "reqId": req_id,
            "retCode": -1,
            "retMsg": "trading WS disconnected before response",
        }));
    }
}

/// Read messages until disconnect. Returns `true` if reconnect should be attempted.
async fn read_until_disconnect(
    read: &mut RawReader,
    shutdown: &mut broadcast::Receiver<()>,
    pending: &Arc<RwLock<HashMap<String, Callback>>>,
    connected: &Arc<AtomicBool>,
    authenticated: &Arc<AtomicBool>,
) -> bool {
    loop {
        tokio::select! {
            msg = read.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        // Dispatch inline: spawning a task per message loses the
                        // stream's ordering guarantee across responses.
                        dispatch_trading_message(&text, pending, authenticated).await;
                    }
                    Some(Ok(Message::Close(_))) => {
                        tracing::debug!("Trading WS closed by server");
                        connected.store(false, Ordering::SeqCst);
                        return true; // Try to reconnect
                    }
                    Some(Ok(_)) => {}
                    Some(Err(e)) => {
                        tracing::error!("Trading WS read error: {}", e);
                        connected.store(false, Ordering::SeqCst);
                        return true; // Try to reconnect
                    }
                    None => {
                        tracing::debug!("Trading WS stream ended");
                        connected.store(false, Ordering::SeqCst);
                        return true; // Try to reconnect
                    }
                }
            }
            _ = shutdown.recv() => {
                tracing::debug!("Trading read loop shutting down");
                connected.store(false, Ordering::SeqCst);
                return false; // Graceful shutdown — don't reconnect
            }
        }
    }
}

/// Perform a trading reconnection: handshake, then re-authenticate.
async fn reconnect_trading(
    url: &str,
    writer: &WsWriter,
    connected: &Arc<AtomicBool>,
    authenticated: &Arc<AtomicBool>,
    pending: &Arc<RwLock<HashMap<String, Callback>>>,
    config: &Arc<WsConfig>,
) -> Result<RawReader> {
    tracing::debug!("Reconnecting trading WS to {}", url);

    let (ws_stream, _response) =
        tokio_tungstenite::connect_async_with_config(url, None, true)
            .await
            .map_err(BybitError::WebSocket)?;

    let (write, mut read) = ws_stream.split();

    // Store the new writer.
    {
        let mut w = writer.lock().await;
        *w = Some(write);
    }
    connected.store(true, Ordering::SeqCst);
    authenticated.store(false, Ordering::SeqCst);

    // Re-authenticate. The caller's read loop is not running yet, so read
    // messages inline here until the auth response arrives. Dispatch inline
    // (awaited) so the `authenticated` flag is set before we re-check it.
    let auth_msg =
        auth::build_ws_auth_message(&config.client_config, config.private_auth_expire)?;
    {
        let mut w = writer.lock().await;
        match w.as_mut() {
            Some(wr) => wr
                .send(Message::Text(auth_msg))
                .await
                .map_err(BybitError::WebSocket)?,
            None => {
                return Err(BybitError::WebSocket(
                    tokio_tungstenite::tungstenite::Error::AlreadyClosed,
                ))
            }
        }
    }

    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    loop {
        tokio::select! {
            msg = read.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        dispatch_trading_message(&text, pending, authenticated).await;
                    }
                    Some(Ok(Message::Close(_))) => {
                        tracing::warn!("Trading connection closed during re-auth");
                        break;
                    }
                    Some(Ok(_)) => {}
                    Some(Err(e)) => return Err(BybitError::WebSocket(e)),
                    None => break,
                }
                if authenticated.load(Ordering::SeqCst) {
                    tracing::debug!("Trading re-authentication successful");
                    break;
                }
            }
            _ = tokio::time::sleep_until(deadline) => {
                tracing::warn!("Trading re-authentication timed out");
                break;
            }
        }
    }

    Ok(read)
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
    async fn fail_pending_invokes_callbacks_with_error() {
        use std::sync::atomic::AtomicU32;

        let pending = Arc::new(RwLock::new(HashMap::new()));
        let count = Arc::new(AtomicU32::new(0));

        let counter = Arc::clone(&count);
        let cb: Callback = Arc::new(move |msg: Value| {
            assert_ne!(msg["retCode"], 0);
            counter.fetch_add(1, Ordering::SeqCst);
        });
        {
            let mut p = pending.write().await;
            p.insert("req-1".into(), Arc::clone(&cb));
            p.insert("req-2".into(), cb);
        }

        fail_pending_requests(&pending).await;

        assert_eq!(count.load(Ordering::SeqCst), 2);
        assert!(pending.read().await.is_empty());
    }

    #[tokio::test]
    async fn dispatch_pong_swallowed() {
        let pending = Arc::new(RwLock::new(HashMap::new()));
        let authenticated = Arc::new(AtomicBool::new(false));

        let msg = r#"{"op":"pong","ret_msg":"pong"}"#;
        dispatch_trading_message(msg, &pending, &authenticated).await;
        // No crash, no side effects
    }

    #[test]
    fn format_based_message_is_valid_json() {
        let req_id = "42";
        let timestamp = 1234567890000u64;
        let operation = "order.create";
        let params_json = r#"{"category":"linear","symbol":"BTCUSDT"}"#;

        let msg = format!(
            r#"{{"reqId":"{}","header":{{"X-BAPI-TIMESTAMP":{}}},"op":"{}","args":[{}]}}"#,
            req_id, timestamp, operation, params_json
        );

        let parsed: Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["reqId"], "42");
        assert_eq!(parsed["op"], "order.create");
        assert_eq!(parsed["header"]["X-BAPI-TIMESTAMP"], 1234567890000u64);
        assert_eq!(parsed["args"][0]["symbol"], "BTCUSDT");
    }

    #[test]
    fn format_based_message_with_all_headers() {
        let req_id = "99";
        let timestamp = 1234567890000u64;
        let recv_window = 5000u32;
        let referral_id = "my-ref";
        let operation = "order.amend";
        let params_json = r#"{"symbol":"ETHUSDT"}"#;

        let header = format!(
            r#"{{"X-BAPI-TIMESTAMP":{},"X-BAPI-RECV-WINDOW":{},"Referer":"{}"}}"#,
            timestamp, recv_window, referral_id
        );
        let msg = format!(
            r#"{{"reqId":"{}","header":{},"op":"{}","args":[{}]}}"#,
            req_id, header, operation, params_json
        );

        let parsed: Value = serde_json::from_str(&msg).unwrap();
        assert_eq!(parsed["reqId"], "99");
        assert_eq!(parsed["header"]["X-BAPI-RECV-WINDOW"], 5000);
        assert_eq!(parsed["header"]["Referer"], "my-ref");
        assert_eq!(parsed["op"], "order.amend");
    }
}
