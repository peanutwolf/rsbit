//! WebSocket connection manager — mirrors Python's `_WebSocketManager` and
//! `_V5WebSocketManager`.
//!
//! Handles connection lifecycle, authentication, ping/pong keepalive,
//! subscription management, and message dispatch to topic callbacks.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::sync::{broadcast, Mutex, RwLock};
use tokio_tungstenite::tungstenite::Message;

use crate::auth;
use crate::config::ClientConfig;
use crate::error::{BybitError, Result};
use crate::ws::subscription::{Callback, SubscriptionRegistry};
use crate::ws::ws_url;

/// WebSocket-specific configuration.
///
/// Wraps a [`ClientConfig`] with WS-specific settings for ping interval,
/// authentication expiry, and channel type.
#[derive(Debug, Clone)]
pub struct WsConfig {
    /// Base client config (auth keys, domain, testnet, etc.).
    pub client_config: ClientConfig,
    /// Channel type: `"linear"`, `"spot"`, `"inverse"`, `"option"`, `"private"`, etc.
    pub channel_type: String,
    /// Interval between application-level `{"op":"ping"}` messages.
    pub ping_interval: Duration,
    /// Seconds added to current timestamp for auth message expiry.
    pub private_auth_expire: u64,
    /// Maximum number of reconnection attempts. 0 means infinite retries.
    pub max_retries: u32,
    /// Whether to automatically reconnect on connection errors.
    pub restart_on_error: bool,
}

impl WsConfig {
    /// Create a WS config for a public channel.
    pub fn public(channel_type: impl Into<String>) -> Self {
        Self {
            client_config: ClientConfig::default(),
            channel_type: channel_type.into(),
            ping_interval: Duration::from_secs(20),
            private_auth_expire: 1,
            max_retries: 10,
            restart_on_error: true,
        }
    }

    /// Create a WS config for a public channel on testnet.
    pub fn public_testnet(channel_type: impl Into<String>) -> Self {
        Self {
            client_config: ClientConfig {
                testnet: true,
                ..ClientConfig::default()
            },
            channel_type: channel_type.into(),
            ping_interval: Duration::from_secs(20),
            private_auth_expire: 1,
            max_retries: 10,
            restart_on_error: true,
        }
    }

    /// Create a WS config for the private channel.
    pub fn private(api_key: impl Into<String>, api_secret: impl Into<String>) -> Self {
        Self {
            client_config: ClientConfig::mainnet(api_key, api_secret),
            channel_type: "private".into(),
            ping_interval: Duration::from_secs(20),
            private_auth_expire: 1,
            max_retries: 10,
            restart_on_error: true,
        }
    }

    /// Create a WS config for the private testnet channel.
    pub fn private_testnet(api_key: impl Into<String>, api_secret: impl Into<String>) -> Self {
        Self {
            client_config: ClientConfig::testnet(api_key, api_secret),
            channel_type: "private".into(),
            ping_interval: Duration::from_secs(20),
            private_auth_expire: 1,
            max_retries: 10,
            restart_on_error: true,
        }
    }

    /// Create a WS config for the trading channel (mainnet).
    pub fn trade(api_key: impl Into<String>, api_secret: impl Into<String>) -> Self {
        Self {
            client_config: ClientConfig::mainnet(api_key, api_secret),
            channel_type: "trade".into(),
            ping_interval: Duration::from_secs(20),
            private_auth_expire: 1,
            max_retries: 10,
            restart_on_error: true,
        }
    }

    /// Create a WS config for the trading channel (testnet).
    pub fn trade_testnet(api_key: impl Into<String>, api_secret: impl Into<String>) -> Self {
        Self {
            client_config: ClientConfig::testnet(api_key, api_secret),
            channel_type: "trade".into(),
            ping_interval: Duration::from_secs(20),
            private_auth_expire: 1,
            max_retries: 10,
            restart_on_error: true,
        }
    }

    /// Set a custom ping interval.
    pub fn with_ping_interval(mut self, interval: Duration) -> Self {
        self.ping_interval = interval;
        self
    }

    /// Set the auth expiry offset in seconds.
    pub fn with_private_auth_expire(mut self, secs: u64) -> Self {
        self.private_auth_expire = secs;
        self
    }

    /// Set the maximum number of reconnection attempts. 0 means infinite retries.
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Enable or disable automatic reconnection on errors.
    pub fn with_restart_on_error(mut self, restart: bool) -> Self {
        self.restart_on_error = restart;
        self
    }

    /// Whether this is an authenticated channel (private or trade).
    pub fn is_private(&self) -> bool {
        self.channel_type == "private" || self.channel_type == "trade"
    }
}

/// The raw WebSocket write-half type.
pub(crate) type RawWriter = futures_util::stream::SplitSink<
    tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
    Message,
>;

/// Type alias for the WebSocket write half, protected by a mutex.
/// `None` when not connected.
pub(crate) type WsWriter = Arc<Mutex<Option<RawWriter>>>;

/// Bybit V5 WebSocket connection manager.
///
/// Manages the WebSocket lifecycle: connect, authenticate, subscribe,
/// receive messages, dispatch to callbacks, and ping keepalive.
/// Automatically reconnects on connection loss when `restart_on_error` is enabled.
///
/// # Example
///
/// ```rust,no_run
/// use std::sync::Arc;
/// use rsbit::ws::manager::{WsConfig, WsManager};
///
/// # async fn run() -> rsbit::error::Result<()> {
/// let config = WsConfig::public_testnet("linear");
/// let ws = WsManager::new(config);
///
/// ws.connect().await?;
/// ws.subscribe("tickers.BTCUSDT", Arc::new(|msg| {
///     println!("Ticker: {}", msg);
/// })).await?;
/// # Ok(())
/// # }
/// ```
pub struct WsManager {
    config: Arc<WsConfig>,
    url: String,
    registry: Arc<RwLock<SubscriptionRegistry>>,
    writer: WsWriter,
    connected: Arc<AtomicBool>,
    authenticated: Arc<AtomicBool>,
    shutdown_tx: broadcast::Sender<()>,
}

impl WsManager {
    /// Create a new WebSocket manager. Does not connect yet — call [`connect`](Self::connect).
    pub fn new(config: WsConfig) -> Self {
        let url = ws_url(&config.client_config, &config.channel_type);
        let (shutdown_tx, _) = broadcast::channel(4);

        Self {
            config: Arc::new(config),
            url,
            registry: Arc::new(RwLock::new(SubscriptionRegistry::new())),
            writer: Arc::new(Mutex::new(None)),
            connected: Arc::new(AtomicBool::new(false)),
            authenticated: Arc::new(AtomicBool::new(false)),
            shutdown_tx,
        }
    }

    /// Connect to the WebSocket server.
    ///
    /// Performs the TLS/WS handshake, authenticates if private, resubscribes
    /// any stored subscriptions, and starts the ping and read-loop tasks.
    /// The read loop automatically reconnects on connection loss when
    /// `restart_on_error` is enabled in the config.
    pub async fn connect(&self) -> Result<()> {
        self.establish_connection().await?;

        // Spawn the ping task
        spawn_ping_task(
            Arc::clone(&self.writer),
            self.config.ping_interval,
            Arc::clone(&self.connected),
            self.shutdown_tx.subscribe(),
        );

        Ok(())
    }

    /// Perform the WebSocket handshake, spawn the read loop, authenticate
    /// if private, and resubscribe.
    async fn establish_connection(&self) -> Result<()> {
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

        // Spawn the read loop BEFORE authenticating so that auth responses
        // are processed (mirrors pybit's approach where run_forever() starts
        // the read thread before _auth() is called).
        let read_registry = Arc::clone(&self.registry);
        let read_connected = Arc::clone(&self.connected);
        let read_authenticated = Arc::clone(&self.authenticated);
        let read_writer = Arc::clone(&self.writer);
        let read_shutdown_tx = self.shutdown_tx.clone();
        let mut read_shutdown = self.shutdown_tx.subscribe();
        let config = Arc::clone(&self.config);
        let url = self.url.clone();
        let log_requests = self.config.client_config.log_requests;

        tokio::spawn(async move {
            run_read_loop(
                read,
                &mut read_shutdown,
                &read_registry,
                &read_connected,
                &read_authenticated,
                &read_writer,
                &read_shutdown_tx,
                &config,
                &url,
                log_requests,
            )
            .await;
        });

        // Authenticate if private channel
        if self.config.is_private() {
            self.authenticate().await?;
        }

        // Resubscribe any stored subscriptions (from a previous connection)
        self.resubscribe().await?;

        Ok(())
    }

    /// Subscribe to a topic with a callback.
    ///
    /// Sends the subscribe message to the server and stores the subscription
    /// for automatic resubscription on reconnect.
    pub async fn subscribe(
        &self,
        topic: impl Into<String>,
        callback: Callback,
    ) -> Result<()> {
        let topic = topic.into();

        // Check not already subscribed
        {
            let reg = self.registry.read().await;
            if reg.has_topic(&topic) {
                return Err(BybitError::TopicMismatch(format!(
                    "Already subscribed to: {}",
                    topic
                )));
            }
        }

        let req_id = super::next_req_id();
        let subscribe_msg = serde_json::json!({
            "op": "subscribe",
            "req_id": req_id,
            "args": [&topic]
        })
        .to_string();

        // Send to server
        self.send_raw(&subscribe_msg).await?;

        // Store in registry
        {
            let mut reg = self.registry.write().await;
            reg.add(req_id, subscribe_msg, &[topic], callback);
        }

        Ok(())
    }

    /// Unsubscribe from a topic.
    ///
    /// Sends the unsubscribe message and removes the callback.
    pub async fn unsubscribe(&self, topic: &str) -> Result<()> {
        let unsubscribe_msg = serde_json::json!({
            "op": "unsubscribe",
            "args": [topic]
        })
        .to_string();

        self.send_raw(&unsubscribe_msg).await?;

        {
            let mut reg = self.registry.write().await;
            reg.remove(topic);
        }

        Ok(())
    }

    /// Check if the WebSocket is currently connected.
    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    /// Retrieve all currently subscribed topic strings.
    pub async fn get_subscription_topics(&self) -> Vec<String> {
        let reg = self.registry.read().await;
        reg.all_topics().into_iter().map(|s| s.to_string()).collect()
    }

    /// Gracefully close the WebSocket connection.
    pub async fn close(&self) {
        // Signal all tasks to stop
        let _ = self.shutdown_tx.send(());

        // Send close frame
        {
            let mut w = self.writer.lock().await;
            if let Some(ref mut writer) = *w {
                let _ = writer.send(Message::Close(None)).await;
                *w = None;
            }
        }

        self.connected.store(false, Ordering::SeqCst);
        self.authenticated.store(false, Ordering::SeqCst);
        tracing::debug!("WebSocket closed");
    }

    /// Send the auth message for private channels.
    async fn authenticate(&self) -> Result<()> {
        let auth_msg = auth::build_ws_auth_message(
            &self.config.client_config,
            self.config.private_auth_expire,
        )?;

        tracing::debug!("Sending auth message");
        self.send_raw(&auth_msg).await?;

        // Wait for auth response (up to 5 seconds)
        for _ in 0..50 {
            if self.authenticated.load(Ordering::SeqCst) {
                tracing::debug!("Authentication successful");
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

    /// Resubscribe all stored subscriptions (after reconnect).
    async fn resubscribe(&self) -> Result<()> {
        let messages: Vec<String> = {
            let reg = self.registry.read().await;
            reg.all_messages().into_iter().map(|s| s.to_string()).collect()
        };

        for msg in messages {
            self.send_raw(&msg).await?;
        }

        Ok(())
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

impl std::fmt::Debug for WsManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WsManager")
            .field("url", &self.url)
            .field("connected", &self.connected.load(Ordering::SeqCst))
            .field("authenticated", &self.authenticated.load(Ordering::SeqCst))
            .finish()
    }
}

/// Dispatch a received WebSocket text message.
///
/// Handles pong responses, auth/subscribe/unsubscribe confirmations,
/// and routes data messages to topic callbacks.
async fn dispatch_message(
    text: &str,
    registry: &Arc<RwLock<SubscriptionRegistry>>,
    authenticated: &Arc<AtomicBool>,
    log: bool,
) {
    let msg: Value = match serde_json::from_str(text) {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!("Failed to parse WS message: {} — raw: {}", e, text);
            return;
        }
    };

    // 1. Pong response — swallow silently
    if is_pong(&msg) {
        return;
    }

    let op = msg.get("op").and_then(|v| v.as_str()).unwrap_or("");

    // 2. Auth response
    if op == "auth" || msg.get("type").and_then(|v| v.as_str()) == Some("AUTH_RESP") {
        let success = msg.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
        if success {
            authenticated.store(true, Ordering::SeqCst);
            tracing::debug!("Auth confirmed");
        } else {
            tracing::error!("Auth failed: {}", msg);
        }
        return;
    }

    // 3. Subscribe confirmation
    if op == "subscribe" || msg.get("type").and_then(|v| v.as_str()) == Some("COMMAND_RESP") {
        let success = msg.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
        if success {
            if log {
                tracing::debug!("Subscription confirmed: {}", msg);
            }
        } else {
            tracing::warn!("Subscription failed: {}", msg);
        }
        return;
    }

    // 4. Unsubscribe confirmation
    if op == "unsubscribe" {
        if log {
            tracing::debug!("Unsubscription confirmed: {}", msg);
        }
        return;
    }

    // 5. Data message — route by topic
    if let Some(topic) = msg.get("topic").and_then(|v| v.as_str()) {
        let reg = registry.read().await;
        if let Some(callback) = reg.get_callback(topic) {
            let cb = Arc::clone(callback);
            let msg_owned = msg.clone();
            // Call callback outside the lock to avoid deadlocks
            drop(reg);
            cb(msg_owned);
        } else if log {
            tracing::debug!("No callback for topic: {}", topic);
        }
        return;
    }

    // 6. Unknown message
    if log {
        tracing::debug!("Unhandled WS message: {}", msg);
    }
}

/// The raw WebSocket read-half type.
type RawReader = futures_util::stream::SplitStream<
    tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
>;

/// Spawn the ping keepalive task.
pub(crate) fn spawn_ping_task(
    writer: WsWriter,
    interval: Duration,
    connected: Arc<AtomicBool>,
    mut shutdown: broadcast::Receiver<()>,
) {
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = tokio::time::sleep(interval) => {
                    if !connected.load(Ordering::SeqCst) {
                        break;
                    }
                    let mut w = writer.lock().await;
                    if let Some(ref mut wr) = *w {
                        let ping_msg = r#"{"op":"ping"}"#;
                        if let Err(e) = wr.send(Message::Text(ping_msg.into())).await {
                            tracing::warn!("Ping send failed: {}", e);
                            break;
                        }
                    }
                }
                _ = shutdown.recv() => {
                    tracing::debug!("Ping task shutting down");
                    break;
                }
            }
        }
    });
}

/// Run the read loop with automatic reconnection on disconnect.
///
/// When the connection drops and `restart_on_error` is enabled, this function
/// attempts to reconnect up to `max_retries` times (0 = infinite), re-authenticating
/// and resubscribing on each successful reconnection.
async fn run_read_loop(
    mut read: RawReader,
    shutdown: &mut broadcast::Receiver<()>,
    registry: &Arc<RwLock<SubscriptionRegistry>>,
    connected: &Arc<AtomicBool>,
    authenticated: &Arc<AtomicBool>,
    writer: &WsWriter,
    shutdown_tx: &broadcast::Sender<()>,
    config: &Arc<WsConfig>,
    url: &str,
    log: bool,
) {
    let mut retry_count: u32 = 0;

    loop {
        // Inner read loop — processes messages until disconnect
        let should_reconnect = read_until_disconnect(
            &mut read,
            shutdown,
            registry,
            connected,
            authenticated,
            log,
        )
        .await;

        if !should_reconnect || !config.restart_on_error {
            break;
        }

        // Check retry limit (0 = infinite)
        if config.max_retries > 0 {
            retry_count += 1;
            if retry_count > config.max_retries {
                tracing::error!(
                    "Max reconnect attempts ({}) exceeded",
                    config.max_retries
                );
                break;
            }
        }

        // Exponential backoff: 1s, 2s, 4s, 8s, ... capped at 30s
        let delay = Duration::from_secs(
            (1u64 << retry_count.min(4)).min(30),
        );
        tracing::info!(
            "Reconnecting in {:?} (attempt {}{})",
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
                tracing::debug!("Reconnect cancelled by shutdown");
                break;
            }
        }

        // Attempt reconnection
        match reconnect(url, writer, connected, authenticated, config, registry).await {
            Ok(new_read) => {
                read = new_read;
                retry_count = 0; // Reset on successful reconnect
                tracing::info!("Reconnected successfully");

                // Spawn a fresh ping task for this new connection
                spawn_ping_task(
                    Arc::clone(writer),
                    config.ping_interval,
                    Arc::clone(connected),
                    shutdown_tx.subscribe(),
                );
            }
            Err(e) => {
                tracing::error!("Reconnect failed: {}", e);
                // Loop continues — will retry after backoff
            }
        }
    }
}

/// Read messages until disconnect. Returns `true` if reconnect should be attempted.
async fn read_until_disconnect(
    read: &mut RawReader,
    shutdown: &mut broadcast::Receiver<()>,
    registry: &Arc<RwLock<SubscriptionRegistry>>,
    connected: &Arc<AtomicBool>,
    authenticated: &Arc<AtomicBool>,
    log: bool,
) -> bool {
    loop {
        tokio::select! {
            msg = read.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        // Dispatch inline: spawning a task per message loses the
                        // stream's ordering guarantee (e.g. New after Cancelled).
                        dispatch_message(&text, registry, authenticated, log).await;
                    }
                    Some(Ok(Message::Close(_))) => {
                        tracing::debug!("WebSocket closed by server");
                        connected.store(false, Ordering::SeqCst);
                        return true; // Try to reconnect
                    }
                    Some(Ok(_)) => {
                        // Binary, Ping, Pong frames — ignore
                    }
                    Some(Err(e)) => {
                        tracing::error!("WebSocket read error: {}", e);
                        connected.store(false, Ordering::SeqCst);
                        return true; // Try to reconnect
                    }
                    None => {
                        tracing::debug!("WebSocket stream ended");
                        connected.store(false, Ordering::SeqCst);
                        return true; // Try to reconnect
                    }
                }
            }
            _ = shutdown.recv() => {
                tracing::debug!("Read loop shutting down");
                connected.store(false, Ordering::SeqCst);
                return false; // Graceful shutdown — don't reconnect
            }
        }
    }
}

/// Perform a reconnection: handshake, auth, resubscribe.
async fn reconnect(
    url: &str,
    writer: &WsWriter,
    connected: &Arc<AtomicBool>,
    authenticated: &Arc<AtomicBool>,
    config: &Arc<WsConfig>,
    registry: &Arc<RwLock<SubscriptionRegistry>>,
) -> Result<RawReader> {
    tracing::debug!("Reconnecting to {}", url);

    let (ws_stream, _response) =
        tokio_tungstenite::connect_async_with_config(url, None, true)
            .await
            .map_err(BybitError::WebSocket)?;

    let (write, mut read) = ws_stream.split();

    // Store new writer
    {
        let mut w = writer.lock().await;
        *w = Some(write);
    }
    connected.store(true, Ordering::SeqCst);
    authenticated.store(false, Ordering::SeqCst);

    // Helper to send raw messages through the writer
    let send = |msg: &str| {
        let writer = Arc::clone(writer);
        let msg = msg.to_string();
        async move {
            let mut w = writer.lock().await;
            match w.as_mut() {
                Some(wr) => wr
                    .send(Message::Text(msg.into()))
                    .await
                    .map_err(BybitError::WebSocket),
                None => Err(BybitError::WebSocket(
                    tokio_tungstenite::tungstenite::Error::AlreadyClosed,
                )),
            }
        }
    };

    // Authenticate if private channel.
    // Read messages inline while waiting for the auth response, since the
    // caller's read loop is not running during reconnection.
    if config.is_private() {
        let auth_msg = auth::build_ws_auth_message(
            &config.client_config,
            config.private_auth_expire,
        )?;
        send(&auth_msg).await?;

        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        loop {
            tokio::select! {
                msg = read.next() => {
                    match msg {
                        Some(Ok(Message::Text(text))) => {
                            dispatch_message(&text, registry, authenticated, false).await;
                        }
                        Some(Ok(Message::Close(_))) => {
                            tracing::warn!("Connection closed during re-auth");
                            break;
                        }
                        Some(Ok(_)) => {}
                        Some(Err(e)) => {
                            return Err(BybitError::WebSocket(e));
                        }
                        None => break,
                    }
                    if authenticated.load(Ordering::SeqCst) {
                        tracing::debug!("Re-authentication successful");
                        break;
                    }
                }
                _ = tokio::time::sleep_until(deadline) => {
                    tracing::warn!("Re-authentication timed out");
                    break;
                }
            }
        }
    }

    // Resubscribe all stored subscriptions
    let messages: Vec<String> = {
        let reg = registry.read().await;
        reg.all_messages().into_iter().map(|s| s.to_string()).collect()
    };
    for msg in messages {
        send(&msg).await?;
    }

    Ok(read)
}

/// Check if a message is a pong response.
///
/// Bybit sends pong as either `{"ret_msg":"pong",...}` or `{"op":"pong",...}`.
pub(crate) fn is_pong(msg: &Value) -> bool {
    if let Some(ret_msg) = msg.get("ret_msg").and_then(|v| v.as_str()) {
        if ret_msg == "pong" {
            return true;
        }
    }
    if let Some(op) = msg.get("op").and_then(|v| v.as_str()) {
        if op == "pong" {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_pong_op() {
        let msg: Value = serde_json::json!({"op": "pong"});
        assert!(is_pong(&msg));
    }

    #[test]
    fn is_pong_ret_msg() {
        let msg: Value = serde_json::json!({"ret_msg": "pong", "op": "", "conn_id": "abc"});
        assert!(is_pong(&msg));
    }

    #[test]
    fn is_not_pong() {
        let msg: Value = serde_json::json!({"op": "subscribe", "success": true});
        assert!(!is_pong(&msg));
    }

    #[test]
    fn ws_config_public_testnet() {
        let config = WsConfig::public_testnet("linear");
        assert!(config.client_config.testnet);
        assert_eq!(config.channel_type, "linear");
        assert!(!config.is_private());
    }

    #[test]
    fn ws_config_private() {
        let config = WsConfig::private("key", "secret");
        assert!(config.is_private());
        assert_eq!(
            config.client_config.api_key.as_deref(),
            Some("key")
        );
    }

    #[test]
    fn ws_manager_new_does_not_connect() {
        let config = WsConfig::public("linear");
        let ws = WsManager::new(config);
        assert!(!ws.is_connected());
        assert_eq!(ws.url, "wss://stream.bybit.com/v5/public/linear");
    }

    #[test]
    fn ws_manager_testnet_url() {
        let config = WsConfig::public_testnet("spot");
        let ws = WsManager::new(config);
        assert_eq!(ws.url, "wss://stream-testnet.bybit.com/v5/public/spot");
    }

    #[test]
    fn ws_manager_private_url() {
        let config = WsConfig::private_testnet("k", "s");
        let ws = WsManager::new(config);
        assert_eq!(ws.url, "wss://stream-testnet.bybit.com/v5/private");
    }

    #[test]
    fn ping_message_is_valid_json() {
        let ping_msg = r#"{"op":"ping"}"#;
        let parsed: Value = serde_json::from_str(ping_msg).unwrap();
        assert_eq!(parsed.get("op").and_then(|v| v.as_str()), Some("ping"));
    }

    #[test]
    fn ws_config_defaults() {
        let config = WsConfig::public("linear");
        assert_eq!(config.max_retries, 10);
        assert!(config.restart_on_error);
        assert_eq!(config.ping_interval, Duration::from_secs(20));
    }

    #[test]
    fn ws_config_with_max_retries() {
        let config = WsConfig::public("linear").with_max_retries(5);
        assert_eq!(config.max_retries, 5);
    }

    #[test]
    fn ws_config_with_restart_disabled() {
        let config = WsConfig::public("linear").with_restart_on_error(false);
        assert!(!config.restart_on_error);
    }

    #[test]
    fn ws_config_infinite_retries() {
        let config = WsConfig::public("linear").with_max_retries(0);
        assert_eq!(config.max_retries, 0);
    }
}
