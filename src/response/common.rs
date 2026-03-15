//! Common response envelope types shared across all Bybit V5 API responses.

use serde::Deserialize;
use serde_json::Value;

/// The standard Bybit V5 API response envelope.
///
/// Every endpoint returns this shape:
/// ```json
/// { "retCode": 0, "retMsg": "OK", "result": { ... }, "retExtInfo": {}, "time": 1234567890123 }
/// ```
///
/// Use `BybitResponse<T>` where `T` is the typed `result` payload for that endpoint.
/// For untyped/dynamic usage, use `BybitResponse<Value>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BybitResponse<T> {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: T,
    #[serde(default)]
    pub ret_ext_info: Value,
    #[serde(default)]
    pub time: Option<u64>,
}

impl<T> BybitResponse<T> {
    /// Returns `true` if the API returned success (`retCode == 0`).
    pub fn is_ok(&self) -> bool {
        self.ret_code == 0
    }
}

/// A paginated list result — used by many endpoints that return `{ "list": [...], "nextPageCursor": "..." }`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedList<T> {
    pub list: Vec<T>,
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// A categorized paginated list — used by endpoints that include `category` in the result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategorizedList<T> {
    #[serde(default)]
    pub category: Option<String>,
    pub list: Vec<T>,
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// A categorized list with a symbol field (used by kline, open-interest, etc.).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolList<T> {
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    pub list: Vec<T>,
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bybit_response_deserialize() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {"timeSecond": "1234", "timeNano": "1234000"},
            "retExtInfo": {},
            "time": 1234567890123
        }"#;
        let resp: BybitResponse<Value> = serde_json::from_str(json).unwrap();
        assert!(resp.is_ok());
        assert_eq!(resp.ret_msg, "OK");
        assert_eq!(resp.time, Some(1234567890123));
        assert_eq!(resp.result["timeSecond"], "1234");
    }

    #[test]
    fn test_bybit_response_error() {
        let json = r#"{
            "retCode": 10001,
            "retMsg": "params error",
            "result": {},
            "retExtInfo": {},
            "time": 1234567890123
        }"#;
        let resp: BybitResponse<Value> = serde_json::from_str(json).unwrap();
        assert!(!resp.is_ok());
        assert_eq!(resp.ret_code, 10001);
    }

    #[test]
    fn test_paginated_list_deserialize() {
        let json = r#"{
            "list": [{"id": 1}, {"id": 2}],
            "nextPageCursor": "abc123"
        }"#;
        let result: PaginatedList<Value> = serde_json::from_str(json).unwrap();
        assert_eq!(result.list.len(), 2);
        assert_eq!(result.next_page_cursor.as_deref(), Some("abc123"));
    }

    #[test]
    fn test_categorized_list_deserialize() {
        let json = r#"{
            "category": "linear",
            "list": [{"symbol": "BTCUSDT"}],
            "nextPageCursor": ""
        }"#;
        let result: CategorizedList<Value> = serde_json::from_str(json).unwrap();
        assert_eq!(result.category.as_deref(), Some("linear"));
        assert_eq!(result.list.len(), 1);
    }
}
