//! Delta processing for orderbook and ticker streams.
//!
//! Mirrors Python's `_process_delta_orderbook` and `_process_delta_ticker`
//! from `_websocket_stream.py`.
//!
//! Orderbook and ticker streams use a snapshot+delta protocol: the first
//! message is a full snapshot, and subsequent messages contain only changes.
//! These types maintain local state and always pass full snapshots to user
//! callbacks.

use std::sync::{Arc, Mutex};

use serde_json::Value;

use crate::ws::subscription::Callback;

/// Maintains the local orderbook state for a single topic.
///
/// On `"snapshot"` messages, stores the full book.
/// On `"delta"` messages, applies incremental changes (insert/update/delete)
/// and produces a synthetic snapshot for the user callback.
#[derive(Debug, Default)]
pub struct OrderbookState {
    /// The full orderbook data: `{"b": [...], "a": [...], "u": ..., "seq": ...}`
    data: Option<Value>,
}

impl OrderbookState {
    /// Create a new empty orderbook state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Process an incoming message. Returns the full snapshot to pass to the user,
    /// or `None` if the message could not be processed.
    pub fn process(&mut self, message: &Value) -> Option<Value> {
        let msg_type = message.get("type").and_then(|v| v.as_str()).unwrap_or("");

        if msg_type.contains("snapshot") {
            self.data = message.get("data").cloned();
            return self.build_callback_message(message);
        }

        if msg_type.contains("delta") {
            if let Some(ref mut book) = self.data {
                let delta_data = message.get("data")?;

                // Update u and seq fields
                if let Some(u) = delta_data.get("u") {
                    book["u"] = u.clone();
                }
                if let Some(seq) = delta_data.get("seq") {
                    book["seq"] = seq.clone();
                }

                // Apply bid and ask changes
                for side_key in &["b", "a"] {
                    if let Some(entries) = delta_data.get(*side_key).and_then(|v| v.as_array()) {
                        for entry in entries {
                            self.apply_entry(side_key, entry);
                        }
                    }
                }
            }
            return self.build_callback_message(message);
        }

        // Unknown type — pass through as-is
        None
    }

    /// Apply a single delta entry (insert/update/delete) to a side of the book.
    fn apply_entry(&mut self, side: &str, entry: &Value) {
        let book = match self.data.as_mut() {
            Some(b) => b,
            None => return,
        };

        let side_arr = match book.get_mut(side).and_then(|v| v.as_array_mut()) {
            Some(a) => a,
            None => return,
        };

        let entry_arr = match entry.as_array() {
            Some(a) if a.len() >= 2 => a,
            _ => return,
        };

        let price = &entry_arr[0];
        let qty_str = entry_arr[1].as_str().unwrap_or("0");
        let qty_zero = qty_str == "0" || qty_str.parse::<f64>().unwrap_or(0.0) == 0.0;

        // Find existing index by price (element [0])
        let existing_idx = side_arr.iter().position(|level| {
            level
                .as_array()
                .and_then(|a| a.first())
                .map(|p| p == price)
                .unwrap_or(false)
        });

        if qty_zero {
            // Delete
            if let Some(idx) = existing_idx {
                side_arr.remove(idx);
            }
        } else if let Some(idx) = existing_idx {
            // Update
            side_arr[idx] = entry.clone();
        } else {
            // Insert
            side_arr.push(entry.clone());
        }
    }

    /// Build the callback message: clone the original message envelope,
    /// set type to `"snapshot"` and data to the full book.
    fn build_callback_message(&self, original: &Value) -> Option<Value> {
        let data = self.data.as_ref()?;
        let mut msg = original.clone();
        msg["type"] = Value::String("snapshot".into());
        msg["data"] = data.clone();
        Some(msg)
    }
}

/// Maintains the local ticker state for a single topic.
///
/// On `"snapshot"` messages, stores the full ticker data.
/// On `"delta"` messages, merges changed fields into the stored state.
#[derive(Debug, Default)]
pub struct TickerState {
    data: Option<Value>,
}

impl TickerState {
    /// Create a new empty ticker state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Process an incoming message. Returns the full snapshot to pass to the user.
    pub fn process(&mut self, message: &Value) -> Option<Value> {
        let msg_type = message.get("type").and_then(|v| v.as_str()).unwrap_or("");

        if msg_type.contains("snapshot") {
            self.data = message.get("data").cloned();
        } else if msg_type.contains("delta") {
            if let (Some(ref mut stored), Some(delta)) =
                (&mut self.data, message.get("data"))
            {
                if let (Some(stored_obj), Some(delta_obj)) =
                    (stored.as_object_mut(), delta.as_object())
                {
                    for (key, value) in delta_obj {
                        stored_obj.insert(key.clone(), value.clone());
                    }
                }
            }
        }

        self.build_callback_message(message)
    }

    fn build_callback_message(&self, original: &Value) -> Option<Value> {
        let data = self.data.as_ref()?;
        let mut msg = original.clone();
        msg["type"] = Value::String("snapshot".into());
        msg["data"] = data.clone();
        Some(msg)
    }
}

/// Create a callback wrapper that applies orderbook delta processing.
///
/// The returned [`Callback`] intercepts raw messages, maintains local orderbook
/// state, and always passes a full snapshot to the user callback.
pub fn wrap_orderbook_callback(user_callback: Callback) -> Callback {
    let state = Arc::new(Mutex::new(OrderbookState::new()));
    Arc::new(move |msg: Value| {
        let mut s = state.lock().expect("orderbook state lock poisoned");
        if let Some(snapshot_msg) = s.process(&msg) {
            user_callback(snapshot_msg);
        }
    })
}

/// Create a callback wrapper that applies ticker delta processing.
///
/// The returned [`Callback`] intercepts raw messages, maintains local ticker
/// state, and always passes a full snapshot to the user callback.
pub fn wrap_ticker_callback(user_callback: Callback) -> Callback {
    let state = Arc::new(Mutex::new(TickerState::new()));
    Arc::new(move |msg: Value| {
        let mut s = state.lock().expect("ticker state lock poisoned");
        if let Some(snapshot_msg) = s.process(&msg) {
            user_callback(snapshot_msg);
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[test]
    fn orderbook_snapshot_sets_initial_state() {
        let mut state = OrderbookState::new();
        let msg = json!({
            "topic": "orderbook.50.BTCUSDT",
            "type": "snapshot",
            "data": {
                "b": [["50000.0", "1.5"], ["49999.0", "2.0"]],
                "a": [["50001.0", "1.0"], ["50002.0", "3.0"]],
                "u": 1000,
                "seq": 100
            },
            "ts": 1234567890
        });

        let result = state.process(&msg).unwrap();
        assert_eq!(result["type"], "snapshot");
        assert_eq!(result["data"]["b"].as_array().unwrap().len(), 2);
        assert_eq!(result["data"]["a"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn orderbook_delta_deletes_zero_qty() {
        let mut state = OrderbookState::new();
        state.process(&json!({
            "type": "snapshot",
            "data": {
                "b": [["50000.0", "1.5"], ["49999.0", "2.0"]],
                "a": [["50001.0", "1.0"]],
                "u": 1000, "seq": 100
            }
        }));

        let result = state.process(&json!({
            "type": "delta",
            "data": {
                "b": [["49999.0", "0"]],
                "a": [],
                "u": 1001, "seq": 101
            }
        })).unwrap();

        assert_eq!(result["data"]["b"].as_array().unwrap().len(), 1);
        assert_eq!(result["data"]["u"], 1001);
        assert_eq!(result["data"]["seq"], 101);
    }

    #[test]
    fn orderbook_delta_inserts_new_price_level() {
        let mut state = OrderbookState::new();
        state.process(&json!({
            "type": "snapshot",
            "data": { "b": [["50000.0", "1.5"]], "a": [], "u": 1, "seq": 1 }
        }));

        let result = state.process(&json!({
            "type": "delta",
            "data": { "b": [["49998.0", "3.0"]], "a": [], "u": 2, "seq": 2 }
        })).unwrap();

        assert_eq!(result["data"]["b"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn orderbook_delta_updates_existing_qty() {
        let mut state = OrderbookState::new();
        state.process(&json!({
            "type": "snapshot",
            "data": { "b": [["50000.0", "1.5"]], "a": [], "u": 1, "seq": 1 }
        }));

        let result = state.process(&json!({
            "type": "delta",
            "data": { "b": [["50000.0", "5.0"]], "a": [], "u": 2, "seq": 2 }
        })).unwrap();

        let bids = result["data"]["b"].as_array().unwrap();
        assert_eq!(bids.len(), 1);
        assert_eq!(bids[0][1], "5.0");
    }

    #[test]
    fn orderbook_delta_before_snapshot_returns_none() {
        let mut state = OrderbookState::new();
        let result = state.process(&json!({
            "type": "delta",
            "data": { "b": [["50000.0", "1.0"]], "a": [], "u": 1, "seq": 1 }
        }));
        // No snapshot received yet, delta has no base to apply to
        assert!(result.is_none());
    }

    #[test]
    fn ticker_snapshot_then_delta_merges() {
        let mut state = TickerState::new();
        state.process(&json!({
            "type": "snapshot",
            "data": { "symbol": "BTCUSDT", "lastPrice": "50000", "volume24h": "100" }
        }));

        let result = state.process(&json!({
            "type": "delta",
            "data": { "lastPrice": "50100" }
        })).unwrap();

        assert_eq!(result["data"]["lastPrice"], "50100");
        assert_eq!(result["data"]["volume24h"], "100"); // preserved
        assert_eq!(result["data"]["symbol"], "BTCUSDT"); // preserved
    }

    #[test]
    fn ticker_delta_before_snapshot_returns_none() {
        let mut state = TickerState::new();
        let result = state.process(&json!({
            "type": "delta",
            "data": { "lastPrice": "50100" }
        }));
        assert!(result.is_none());
    }

    #[test]
    fn wrap_orderbook_callback_passes_snapshots() {
        let count = Arc::new(AtomicU32::new(0));
        let count_clone = Arc::clone(&count);
        let user_cb: Callback = Arc::new(move |msg: Value| {
            assert_eq!(msg["type"], "snapshot");
            count_clone.fetch_add(1, Ordering::SeqCst);
        });

        let wrapped = wrap_orderbook_callback(user_cb);

        // Send snapshot
        wrapped(json!({
            "type": "snapshot",
            "data": { "b": [], "a": [], "u": 1, "seq": 1 }
        }));

        // Send delta
        wrapped(json!({
            "type": "delta",
            "data": { "b": [], "a": [], "u": 2, "seq": 2 }
        }));

        assert_eq!(count.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn wrap_ticker_callback_passes_snapshots() {
        let count = Arc::new(AtomicU32::new(0));
        let count_clone = Arc::clone(&count);
        let user_cb: Callback = Arc::new(move |msg: Value| {
            assert_eq!(msg["type"], "snapshot");
            count_clone.fetch_add(1, Ordering::SeqCst);
        });

        let wrapped = wrap_ticker_callback(user_cb);

        wrapped(json!({
            "type": "snapshot",
            "data": { "symbol": "BTCUSDT", "lastPrice": "50000" }
        }));

        wrapped(json!({
            "type": "delta",
            "data": { "lastPrice": "50100" }
        }));

        assert_eq!(count.load(Ordering::SeqCst), 2);
    }
}
