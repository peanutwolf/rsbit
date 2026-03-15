//! Request preparation and submission logic.
//!
//! Mirrors Python's `prepare_payload`, `_prepare_request`, `_submit_request`,
//! `_clean_query`, `_check_status_code`, and `_handle_response`.

use std::collections::HashMap;

use chrono::Utc;
use reqwest::header::HeaderMap;
use serde_json::Value;

use super::{ApiResponse, HttpManager};
use crate::auth;
use crate::error::{BybitError, Result};
use crate::helpers::generate_timestamp;

/// HTTP method for API requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
}

impl Method {
    fn as_reqwest(self) -> reqwest::Method {
        match self {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
        }
    }
}

/// Parameters that known fields should be cast to specific types.
/// Mirrors Python's `cast_values()` in `prepare_payload`.
const STRING_PARAMS: &[&str] = &["qty", "price", "triggerPrice", "takeProfit", "stopLoss"];
const INTEGER_PARAMS: &[&str] = &["positionIdx"];

impl HttpManager {
    /// Submit an API request with optional authentication.
    ///
    /// This is the core method that all endpoint functions call.
    /// Mirrors Python's `_submit_request`.
    pub async fn submit_request(
        &self,
        method: Method,
        path: &str,
        query: Option<HashMap<String, Value>>,
        auth: bool,
    ) -> Result<ApiResponse> {
        let query = clean_query(query);
        let mut recv_window = self.config.recv_window;
        let mut retries_remaining = self.config.max_retries;
        let mut last_params = String::new();

        while retries_remaining > 0 {
            retries_remaining -= 1;

            let payload = prepare_payload(method, &query);
            last_params = payload.clone();

            let auth_headers = if auth {
                Some(self.prepare_auth_headers(&payload, recv_window)?)
            } else {
                None
            };

            let full_url = format!("{}{}", self.endpoint, path);
            self.log_request(method, path, &payload, auth_headers.as_ref());

            let result = self
                .execute_request(method, &full_url, &payload, auth_headers)
                .await;

            match result {
                Ok(response) => {
                    // Check HTTP status code
                    if let Err(e) = self.check_status_code(&response, method, path, &payload) {
                        return Err(e);
                    }

                    // Parse JSON response
                    match self.handle_response(response, method, path, &payload).await {
                        Ok(api_response) => return Ok(api_response),
                        Err(RetryOrError::Retry { recv_window_delta }) => {
                            recv_window += recv_window_delta;
                            continue;
                        }
                        Err(RetryOrError::Error(e)) => return Err(e),
                    }
                }
                Err(e) => {
                    if self.handle_network_error(&e, retries_remaining).await {
                        continue;
                    }
                    return Err(e);
                }
            }
        }

        Err(BybitError::FailedRequest {
            request: format!("{} {}: {}", method.as_str(), path, last_params),
            message: "Bad Request. Retries exceeded maximum.".into(),
            status_code: 400,
            time: Utc::now().format("%H:%M:%S").to_string(),
            resp_headers: None,
        })
    }

    /// Build authentication headers for a request.
    fn prepare_auth_headers(&self, payload: &str, recv_window: u64) -> Result<HeaderMap> {
        let api_key = self
            .config
            .api_key
            .as_deref()
            .ok_or(BybitError::Unauthorized)?;
        let api_secret = self
            .config
            .api_secret
            .as_deref()
            .ok_or(BybitError::Unauthorized)?;

        let timestamp = generate_timestamp();
        let param_str = format!("{}{}{}{}", timestamp, api_key, recv_window, payload);
        let signature =
            auth::generate_signature(&self.config.auth_method, api_secret, &param_str)?;

        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/json".parse().unwrap(),
        );
        headers.insert("X-BAPI-API-KEY", api_key.parse().unwrap());
        headers.insert("X-BAPI-SIGN", signature.parse().unwrap());
        headers.insert("X-BAPI-SIGN-TYPE", "2".parse().unwrap());
        headers.insert("X-BAPI-TIMESTAMP", timestamp.to_string().parse().unwrap());
        headers.insert(
            "X-BAPI-RECV-WINDOW",
            recv_window.to_string().parse().unwrap(),
        );

        Ok(headers)
    }

    /// Execute the HTTP request.
    async fn execute_request(
        &self,
        method: Method,
        url: &str,
        payload: &str,
        auth_headers: Option<HeaderMap>,
    ) -> Result<reqwest::Response> {
        let request_url = match method {
            Method::Get if !payload.is_empty() => format!("{}?{}", url, payload),
            _ => url.to_string(),
        };

        let mut builder = self.client.request(method.as_reqwest(), &request_url);

        if let Some(headers) = auth_headers {
            builder = builder.headers(headers);
        }

        if method == Method::Post {
            builder = builder.body(payload.to_string());
        }

        Ok(builder.send().await?)
    }

    /// Check HTTP status code — mirrors Python's `_check_status_code`.
    fn check_status_code(
        &self,
        response: &reqwest::Response,
        method: Method,
        path: &str,
        params: &str,
    ) -> Result<()> {
        let status = response.status().as_u16();
        if status != 200 {
            let error_msg = if status == 403 {
                "You have breached the IP rate limit or your IP is from the USA."
            } else {
                "HTTP status code is not 200."
            };
            tracing::debug!("Non-200 response status: {}", status);
            return Err(BybitError::FailedRequest {
                request: format!("{} {}: {}", method.as_str(), path, params),
                message: error_msg.into(),
                status_code: status as i32,
                time: Utc::now().format("%H:%M:%S").to_string(),
                resp_headers: Some(headers_to_map(response.headers())),
            });
        }
        Ok(())
    }

    /// Handle JSON response and Bybit error codes.
    /// Mirrors Python's `_handle_response`.
    async fn handle_response(
        &self,
        response: reqwest::Response,
        method: Method,
        path: &str,
        params: &str,
    ) -> std::result::Result<ApiResponse, RetryOrError> {
        let elapsed = response
            .extensions()
            .get::<std::time::Instant>()
            .map(|start| start.elapsed());
        let resp_headers = headers_to_map(response.headers());

        let body: Value = response
            .json()
            .await
            .map_err(|e| RetryOrError::Error(BybitError::Http(e)))?;

        // Check retCode
        let ret_code = body.get("retCode").and_then(|v| v.as_i64()).unwrap_or(0) as i32;

        if ret_code != 0 {
            let ret_msg = body
                .get("retMsg")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error")
                .to_string();

            if self.config.retry_codes.contains(&ret_code) {
                let recv_window_delta =
                    super::retry::handle_retryable_error(self, ret_code, &ret_msg, &resp_headers)
                        .await;
                return Err(RetryOrError::Retry { recv_window_delta });
            }

            if !self.config.ignore_codes.contains(&ret_code) {
                return Err(RetryOrError::Error(BybitError::InvalidRequest {
                    request: format!("{} {}: {}", method.as_str(), path, params),
                    message: ret_msg,
                    status_code: ret_code,
                    time: Utc::now().format("%H:%M:%S").to_string(),
                    resp_headers: Some(resp_headers),
                }));
            }
        }

        if self.config.log_requests {
            tracing::debug!("Response headers: {:?}", resp_headers);
        }

        Ok(ApiResponse {
            body,
            elapsed: if self.config.record_request_time {
                elapsed
            } else {
                None
            },
            headers: if self.config.return_response_headers {
                Some(resp_headers)
            } else {
                None
            },
        })
    }

    /// Handle network errors (timeout, SSL, connection).
    /// Returns `true` if the request should be retried.
    async fn handle_network_error(&self, error: &BybitError, retries_remaining: u32) -> bool {
        if self.config.force_retry && retries_remaining > 0 {
            tracing::error!("{}. Retrying...", error);
            tokio::time::sleep(self.config.retry_delay).await;
            true
        } else {
            false
        }
    }

    /// Log the request if logging is enabled.
    fn log_request(
        &self,
        method: Method,
        path: &str,
        params: &str,
        headers: Option<&HeaderMap>,
    ) {
        if self.config.log_requests {
            if !params.is_empty() {
                tracing::debug!(
                    "Request -> {} {}. Body: {}. Headers: {:?}",
                    method.as_str(),
                    path,
                    params,
                    headers
                );
            } else {
                tracing::debug!(
                    "Request -> {} {}. Headers: {:?}",
                    method.as_str(),
                    path,
                    headers
                );
            }
        }
    }
}

/// Internal enum for response handling: either retry or return an error.
enum RetryOrError {
    Retry { recv_window_delta: u64 },
    Error(BybitError),
}

/// Prepare the request payload.
///
/// - GET: sorted key=value query string (matching Python's sorted `parameters.items()`)
/// - POST: JSON string with type-casting for known fields
///
/// Mirrors Python's `prepare_payload`.
pub fn prepare_payload(method: Method, parameters: &HashMap<String, Value>) -> String {
    match method {
        Method::Get => {
            let mut pairs: Vec<_> = parameters
                .iter()
                .filter(|(_, v)| !v.is_null())
                .map(|(k, v)| {
                    let val_str = match v {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        other => other.to_string(),
                    };
                    (k.as_str(), val_str)
                })
                .collect();
            pairs.sort_by_key(|(k, _)| *k);
            pairs
                .into_iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&")
        }
        Method::Post => {
            let mut params = parameters.clone();
            cast_values(&mut params);
            serde_json::to_string(&params).unwrap_or_default()
        }
    }
}

/// Clean query parameters: remove null values, fix whole-number floats to ints.
/// Mirrors Python's `_clean_query`.
fn clean_query(query: Option<HashMap<String, Value>>) -> HashMap<String, Value> {
    let Some(query) = query else {
        return HashMap::new();
    };
    query
        .into_iter()
        .filter(|(_, v)| !v.is_null())
        .map(|(k, v)| {
            let v = match &v {
                Value::Number(n) => {
                    if let Some(f) = n.as_f64() {
                        if f == (f as i64) as f64 {
                            Value::Number(serde_json::Number::from(f as i64))
                        } else {
                            v
                        }
                    } else {
                        v
                    }
                }
                _ => v,
            };
            (k, v)
        })
        .collect()
}

/// Cast known parameter fields to their expected types.
/// Mirrors Python's `cast_values()`.
fn cast_values(params: &mut HashMap<String, Value>) {
    for (key, value) in params.iter_mut() {
        if STRING_PARAMS.contains(&key.as_str()) {
            if !value.is_string() {
                let s = match &*value {
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    other => other.to_string(),
                };
                *value = Value::String(s);
            }
        } else if INTEGER_PARAMS.contains(&key.as_str()) {
            if !value.is_i64() && !value.is_u64() {
                if let Some(n) = value.as_f64() {
                    *value = Value::Number(serde_json::Number::from(n as i64));
                }
            }
        }
    }
}

/// Convert reqwest HeaderMap to HashMap<String, String>.
fn headers_to_map(headers: &reqwest::header::HeaderMap) -> HashMap<String, String> {
    headers
        .iter()
        .map(|(k, v)| {
            (
                k.as_str().to_string(),
                v.to_str().unwrap_or_default().to_string(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepare_get_payload_sorted() {
        let mut params = HashMap::new();
        params.insert("symbol".into(), Value::String("BTCUSDT".into()));
        params.insert("category".into(), Value::String("linear".into()));
        params.insert("limit".into(), Value::Number(50.into()));

        let payload = prepare_payload(Method::Get, &params);
        assert_eq!(payload, "category=linear&limit=50&symbol=BTCUSDT");
    }

    #[test]
    fn prepare_get_payload_skips_null() {
        let mut params = HashMap::new();
        params.insert("symbol".into(), Value::String("BTCUSDT".into()));
        params.insert("cursor".into(), Value::Null);

        let payload = prepare_payload(Method::Get, &params);
        assert_eq!(payload, "symbol=BTCUSDT");
    }

    #[test]
    fn prepare_post_payload_casts_string_params() {
        let mut params = HashMap::new();
        params.insert("symbol".into(), Value::String("BTCUSDT".into()));
        params.insert("qty".into(), Value::Number(serde_json::Number::from_f64(0.5).unwrap()));
        params.insert("price".into(), Value::Number(50000.into()));

        let payload = prepare_payload(Method::Post, &params);
        let parsed: Value = serde_json::from_str(&payload).unwrap();
        // qty and price should be strings after cast
        assert!(parsed["qty"].is_string());
        assert!(parsed["price"].is_string());
    }

    #[test]
    fn prepare_post_payload_casts_integer_params() {
        let mut params = HashMap::new();
        params.insert(
            "positionIdx".into(),
            Value::Number(serde_json::Number::from_f64(1.0).unwrap()),
        );

        let payload = prepare_payload(Method::Post, &params);
        let parsed: Value = serde_json::from_str(&payload).unwrap();
        assert!(parsed["positionIdx"].is_i64());
    }

    #[test]
    fn clean_query_removes_nulls() {
        let mut query = HashMap::new();
        query.insert("symbol".into(), Value::String("BTCUSDT".into()));
        query.insert("cursor".into(), Value::Null);

        let cleaned = clean_query(Some(query));
        assert_eq!(cleaned.len(), 1);
        assert!(cleaned.contains_key("symbol"));
    }

    #[test]
    fn clean_query_fixes_whole_floats() {
        let mut query = HashMap::new();
        query.insert(
            "limit".into(),
            Value::Number(serde_json::Number::from_f64(50.0).unwrap()),
        );

        let cleaned = clean_query(Some(query));
        assert_eq!(cleaned["limit"], Value::Number(50.into()));
    }

    #[test]
    fn clean_query_none_returns_empty() {
        let cleaned = clean_query(None);
        assert!(cleaned.is_empty());
    }

    #[test]
    fn prepare_get_empty_params() {
        let params = HashMap::new();
        let payload = prepare_payload(Method::Get, &params);
        assert_eq!(payload, "");
    }

    #[test]
    fn prepare_get_boolean_params() {
        let mut params = HashMap::new();
        params.insert("isUta".into(), Value::Bool(true));
        params.insert("symbol".into(), Value::String("BTCUSDT".into()));

        let payload = prepare_payload(Method::Get, &params);
        assert_eq!(payload, "isUta=true&symbol=BTCUSDT");
    }

    #[test]
    fn prepare_post_string_params_left_as_string() {
        let mut params = HashMap::new();
        params.insert("qty".into(), Value::String("0.5".into()));
        params.insert("price".into(), Value::String("50000".into()));

        let payload = prepare_payload(Method::Post, &params);
        let parsed: Value = serde_json::from_str(&payload).unwrap();
        // Already strings — should stay as strings
        assert_eq!(parsed["qty"], "0.5");
        assert_eq!(parsed["price"], "50000");
    }

    #[test]
    fn clean_query_preserves_fractional_floats() {
        let mut query = HashMap::new();
        query.insert(
            "price".into(),
            Value::Number(serde_json::Number::from_f64(50000.5).unwrap()),
        );

        let cleaned = clean_query(Some(query));
        // 50000.5 is not a whole number — should stay as float
        assert_eq!(cleaned["price"].as_f64().unwrap(), 50000.5);
    }

    #[test]
    fn cast_values_trigger_price_and_stop_loss() {
        let mut params = HashMap::new();
        params.insert("triggerPrice".into(), Value::Number(100.into()));
        params.insert("stopLoss".into(), Value::Number(90.into()));
        params.insert("takeProfit".into(), Value::Number(110.into()));

        cast_values(&mut params);
        assert!(params["triggerPrice"].is_string());
        assert!(params["stopLoss"].is_string());
        assert!(params["takeProfit"].is_string());
    }
}
