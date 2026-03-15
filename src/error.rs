//! Error types for the Bybit SDK.
//!
//! Maps Python exceptions to Rust error enums:
//! - `FailedRequestError` → `BybitError::FailedRequest`
//! - `InvalidRequestError` → `BybitError::InvalidRequest`
//! - `UnauthorizedExceptionError` → `BybitError::Unauthorized`
//! - `InvalidChannelTypeError` → `BybitError::InvalidChannelType`
//! - `TopicMismatchError` → `BybitError::TopicMismatch`

use std::collections::HashMap;

/// A specialized `Result` type for Bybit operations.
pub type Result<T> = std::result::Result<T, BybitError>;

/// Error type for all Bybit SDK operations.
#[derive(Debug, thiserror::Error)]
pub enum BybitError {
    /// HTTP request failed (non-200 status, timeout, retries exceeded).
    /// Mirrors Python's `FailedRequestError`.
    #[error("{message} (ErrCode: {status_code}) (ErrTime: {time}).\nRequest → {request}.")]
    FailedRequest {
        request: String,
        message: String,
        status_code: i32,
        time: String,
        resp_headers: Option<HashMap<String, String>>,
    },

    /// Bybit API returned a non-zero `retCode`.
    /// Mirrors Python's `InvalidRequestError`.
    #[error("{message} (ErrCode: {status_code}) (ErrTime: {time}).\nRequest → {request}.")]
    InvalidRequest {
        request: String,
        message: String,
        status_code: i32,
        time: String,
        resp_headers: Option<HashMap<String, String>>,
    },

    /// API keys are missing for an authenticated endpoint.
    /// Mirrors Python's `UnauthorizedExceptionError`.
    #[error("Authenticated endpoints require keys.")]
    Unauthorized,

    /// Invalid WebSocket channel type provided.
    /// Mirrors Python's `InvalidChannelTypeError`.
    #[error("Invalid channel type: {0}")]
    InvalidChannelType(String),

    /// WebSocket topic does not match the current channel.
    /// Mirrors Python's `TopicMismatchError`.
    #[error("Topic mismatch: {0}")]
    TopicMismatch(String),

    /// Underlying HTTP client error from `reqwest`.
    #[error(transparent)]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error.
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    /// WebSocket error from `tokio-tungstenite`.
    #[error(transparent)]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    /// RSA cryptography error.
    #[error("RSA error: {0}")]
    Rsa(#[from] rsa::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_request_error_display() {
        let err = BybitError::FailedRequest {
            request: "GET /v5/market/tickers: symbol=BTCUSDT".into(),
            message: "HTTP status code is not 200".into(),
            status_code: 403,
            time: "12:30:45".into(),
            resp_headers: None,
        };
        let msg = err.to_string();
        assert!(msg.contains("ErrCode: 403"));
        assert!(msg.contains("ErrTime: 12:30:45"));
        assert!(msg.contains("GET /v5/market/tickers"));
    }

    #[test]
    fn invalid_request_error_display() {
        let err = BybitError::InvalidRequest {
            request: "POST /v5/order/create: {}".into(),
            message: "Order not found".into(),
            status_code: 110001,
            time: "14:00:00".into(),
            resp_headers: Some(HashMap::new()),
        };
        let msg = err.to_string();
        assert!(msg.contains("ErrCode: 110001"));
        assert!(msg.contains("Order not found"));
    }

    #[test]
    fn unauthorized_error_display() {
        let err = BybitError::Unauthorized;
        assert_eq!(err.to_string(), "Authenticated endpoints require keys.");
    }

    #[test]
    fn error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<BybitError>();
    }
}
