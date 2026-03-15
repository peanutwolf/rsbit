//! Subscription tracking and callback management for WebSocket streams.
//!
//! Mirrors Python's `self.subscriptions` dict and `self.callback_directory`
//! from `_V5WebSocketManager`.

use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;

/// Callback function type for incoming WebSocket messages.
///
/// Receives the parsed JSON message. Called once per matching topic message.
pub type Callback = Arc<dyn Fn(Value) + Send + Sync>;

/// Tracks active subscriptions and their associated callbacks.
///
/// Stores two parallel maps:
/// - `subscriptions`: `req_id → JSON subscribe message` (for resubscription on reconnect)
/// - `callbacks`: `topic → callback function` (for message dispatch)
#[derive(Default)]
pub struct SubscriptionRegistry {
    /// req_id → original JSON subscribe message string.
    /// Used to replay all subscriptions after a reconnect.
    subscriptions: HashMap<String, String>,
    /// topic → callback function.
    /// Used to route incoming data messages to the right handler.
    callbacks: HashMap<String, Callback>,
}

impl SubscriptionRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a subscription.
    ///
    /// - `req_id`: UUID identifying this subscribe request
    /// - `subscribe_message`: the full JSON string sent to the server
    /// - `topics`: the topic strings this subscription covers
    /// - `callback`: function to call when data arrives for these topics
    pub fn add(
        &mut self,
        req_id: String,
        subscribe_message: String,
        topics: &[String],
        callback: Callback,
    ) {
        self.subscriptions
            .insert(req_id, subscribe_message);
        for topic in topics {
            self.callbacks.insert(topic.clone(), Arc::clone(&callback));
        }
    }

    /// Remove a subscription by topic.
    ///
    /// Removes the callback and the corresponding subscription message.
    /// Returns `true` if the topic was found and removed.
    pub fn remove(&mut self, topic: &str) -> bool {
        let removed_callback = self.callbacks.remove(topic).is_some();

        // Find and remove the subscription whose args contain this topic
        let req_id_to_remove: Option<String> = self
            .subscriptions
            .iter()
            .find(|(_, msg)| msg.contains(topic))
            .map(|(id, _)| id.clone());

        if let Some(req_id) = req_id_to_remove {
            self.subscriptions.remove(&req_id);
        }

        removed_callback
    }

    /// Get the callback for a topic, if registered.
    pub fn get_callback(&self, topic: &str) -> Option<&Callback> {
        self.callbacks.get(topic)
    }

    /// Check whether a topic is already subscribed.
    pub fn has_topic(&self, topic: &str) -> bool {
        self.callbacks.contains_key(topic)
    }

    /// Return all stored subscribe messages (for resubscription after reconnect).
    pub fn all_messages(&self) -> Vec<&str> {
        self.subscriptions.values().map(|s| s.as_str()).collect()
    }

    /// Return all subscribed topic strings.
    pub fn all_topics(&self) -> Vec<&str> {
        self.callbacks.keys().map(|s| s.as_str()).collect()
    }

    /// Number of registered topics.
    pub fn topic_count(&self) -> usize {
        self.callbacks.len()
    }

    /// Number of stored subscription messages.
    pub fn subscription_count(&self) -> usize {
        self.subscriptions.len()
    }
}

impl std::fmt::Debug for SubscriptionRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubscriptionRegistry")
            .field("subscriptions", &self.subscriptions.keys().collect::<Vec<_>>())
            .field("callback_topics", &self.callbacks.keys().collect::<Vec<_>>())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    fn make_callback() -> (Callback, Arc<AtomicU32>) {
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);
        let cb: Callback = Arc::new(move |_msg| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });
        (cb, counter)
    }

    #[test]
    fn add_and_lookup() {
        let mut reg = SubscriptionRegistry::new();
        let (cb, _) = make_callback();

        reg.add(
            "req-1".into(),
            r#"{"op":"subscribe","req_id":"req-1","args":["tickers.BTCUSDT"]}"#.into(),
            &["tickers.BTCUSDT".into()],
            cb,
        );

        assert!(reg.has_topic("tickers.BTCUSDT"));
        assert!(!reg.has_topic("tickers.ETHUSDT"));
        assert_eq!(reg.topic_count(), 1);
        assert_eq!(reg.subscription_count(), 1);
    }

    #[test]
    fn callback_is_invoked() {
        let mut reg = SubscriptionRegistry::new();
        let (cb, counter) = make_callback();

        reg.add(
            "req-1".into(),
            "msg".into(),
            &["orderbook.50.BTCUSDT".into()],
            cb,
        );

        let callback = reg.get_callback("orderbook.50.BTCUSDT").unwrap();
        callback(Value::Null);
        callback(Value::Null);
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn remove_topic() {
        let mut reg = SubscriptionRegistry::new();
        let (cb, _) = make_callback();

        reg.add(
            "req-1".into(),
            r#"{"args":["tickers.BTCUSDT"]}"#.into(),
            &["tickers.BTCUSDT".into()],
            cb,
        );

        assert!(reg.remove("tickers.BTCUSDT"));
        assert!(!reg.has_topic("tickers.BTCUSDT"));
        assert_eq!(reg.topic_count(), 0);
        assert_eq!(reg.subscription_count(), 0);
    }

    #[test]
    fn remove_nonexistent_topic() {
        let mut reg = SubscriptionRegistry::new();
        assert!(!reg.remove("nonexistent"));
    }

    #[test]
    fn multiple_topics_same_subscription() {
        let mut reg = SubscriptionRegistry::new();
        let (cb, _) = make_callback();

        reg.add(
            "req-1".into(),
            r#"{"args":["tickers.BTCUSDT","tickers.ETHUSDT"]}"#.into(),
            &["tickers.BTCUSDT".into(), "tickers.ETHUSDT".into()],
            cb,
        );

        assert!(reg.has_topic("tickers.BTCUSDT"));
        assert!(reg.has_topic("tickers.ETHUSDT"));
        assert_eq!(reg.topic_count(), 2);
        assert_eq!(reg.subscription_count(), 1);
    }

    #[test]
    fn all_messages_for_resubscribe() {
        let mut reg = SubscriptionRegistry::new();
        let (cb1, _) = make_callback();
        let (cb2, _) = make_callback();

        reg.add("req-1".into(), "msg-1".into(), &["topic-a".into()], cb1);
        reg.add("req-2".into(), "msg-2".into(), &["topic-b".into()], cb2);

        let messages = reg.all_messages();
        assert_eq!(messages.len(), 2);
        assert!(messages.contains(&"msg-1"));
        assert!(messages.contains(&"msg-2"));
    }

    #[test]
    fn get_callback_missing_returns_none() {
        let reg = SubscriptionRegistry::new();
        assert!(reg.get_callback("nonexistent").is_none());
    }

    #[test]
    fn empty_registry() {
        let reg = SubscriptionRegistry::new();
        assert_eq!(reg.topic_count(), 0);
        assert_eq!(reg.subscription_count(), 0);
        assert!(reg.all_messages().is_empty());
    }
}
