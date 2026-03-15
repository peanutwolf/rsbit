//! Retry and rate-limit handling logic.
//!
//! Mirrors Python's `_handle_retryable_error`, `_handle_network_error`,
//! and `_handle_json_error` from `_http_manager.py`.

use std::collections::HashMap;

use crate::helpers::generate_timestamp;

use super::HttpManager;

/// Handle a retryable Bybit error code.
///
/// Returns the `recv_window` delta to add (non-zero only for error 10002).
///
/// Mirrors Python's `_handle_retryable_error`:
/// - 10002: recv_window error → add 2500ms to recv_window
/// - 10006: rate limit → sleep until `X-Bapi-Limit-Reset-Timestamp`
/// - Other retry codes: sleep for `retry_delay`
pub(super) async fn handle_retryable_error(
    manager: &HttpManager,
    error_code: i32,
    error_msg: &str,
    resp_headers: &HashMap<String, String>,
) -> u64 {
    let mut delay = manager.config().retry_delay;
    let mut recv_window_delta: u64 = 0;

    match error_code {
        10002 => {
            // recv_window error — widen the window
            tracing::error!(
                "{} (ErrCode: {}). Added 2.5 seconds to recv_window. Retrying...",
                error_msg,
                error_code
            );
            recv_window_delta = 2500;
        }
        10006 => {
            // API rate limit — sleep until the reset timestamp
            tracing::error!(
                "{} (ErrCode: {}). Hit the API rate limit. Sleeping then trying again.",
                error_msg,
                error_code
            );

            let now = generate_timestamp();
            let limit_reset_time: u64 = resp_headers
                .get("x-bapi-limit-reset-timestamp")
                .and_then(|v| v.parse().ok())
                .unwrap_or(now + 2000);

            let delay_ms = limit_reset_time.saturating_sub(generate_timestamp());
            if delay_ms > 0 {
                delay = std::time::Duration::from_millis(delay_ms);
            }

            tracing::error!(
                "API rate limit will reset in {} ms. Sleeping...",
                delay_ms
            );
        }
        _ => {
            tracing::error!(
                "{} (ErrCode: {}). Retrying...",
                error_msg,
                error_code
            );
        }
    }

    tokio::time::sleep(delay).await;
    recv_window_delta
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ClientConfig;

    #[tokio::test]
    async fn recv_window_error_returns_delta() {
        let config = ClientConfig::default()
            .with_retry_delay(std::time::Duration::from_millis(1));
        let manager = HttpManager::new(config).unwrap();
        let headers = HashMap::new();

        let delta = handle_retryable_error(&manager, 10002, "recv_window error", &headers).await;
        assert_eq!(delta, 2500);
    }

    #[tokio::test]
    async fn rate_limit_error_returns_zero_delta() {
        let config = ClientConfig::default()
            .with_retry_delay(std::time::Duration::from_millis(1));
        let manager = HttpManager::new(config).unwrap();

        // Simulate a reset timestamp slightly in the future
        let mut headers = HashMap::new();
        let reset_time = generate_timestamp() + 10; // 10ms from now
        headers.insert(
            "x-bapi-limit-reset-timestamp".to_string(),
            reset_time.to_string(),
        );

        let delta = handle_retryable_error(&manager, 10006, "rate limited", &headers).await;
        assert_eq!(delta, 0);
    }

    #[tokio::test]
    async fn other_retry_code_returns_zero_delta() {
        let config = ClientConfig::default()
            .with_retry_delay(std::time::Duration::from_millis(1));
        let manager = HttpManager::new(config).unwrap();
        let headers = HashMap::new();

        let delta = handle_retryable_error(&manager, 30034, "some error", &headers).await;
        assert_eq!(delta, 0);
    }

    #[tokio::test]
    async fn rate_limit_missing_reset_header_uses_fallback() {
        let config = ClientConfig::default()
            .with_retry_delay(std::time::Duration::from_millis(1));
        let manager = HttpManager::new(config).unwrap();
        // No reset timestamp header — should use fallback (now + 2000ms)
        let headers = HashMap::new();

        let delta = handle_retryable_error(&manager, 10006, "rate limited", &headers).await;
        assert_eq!(delta, 0);
    }

    #[tokio::test]
    async fn rate_limit_past_reset_uses_retry_delay() {
        let config = ClientConfig::default()
            .with_retry_delay(std::time::Duration::from_millis(1));
        let manager = HttpManager::new(config).unwrap();

        // Reset timestamp in the past — saturating_sub yields 0, falls through to retry_delay
        let mut headers = HashMap::new();
        headers.insert(
            "x-bapi-limit-reset-timestamp".to_string(),
            "1000".to_string(), // way in the past
        );

        let delta = handle_retryable_error(&manager, 10006, "rate limited", &headers).await;
        assert_eq!(delta, 0);
    }
}
