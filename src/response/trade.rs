//! Trade endpoint response types — mirrors `pybit/_v5_trade.py` responses.

use serde::Deserialize;
use serde_json::Value;

use super::common::{BybitResponse, CategorizedList};

// ── Place/Amend/Cancel Order ────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResult {
    pub order_id: String,
    pub order_link_id: String,
}

pub type PlaceOrderResponse = BybitResponse<OrderResult>;
pub type AmendOrderResponse = BybitResponse<OrderResult>;
pub type CancelOrderResponse = BybitResponse<OrderResult>;

// ── Get Open Orders / Order History ─────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderEntry {
    pub symbol: String,
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub order_link_id: Option<String>,
    #[serde(default)]
    pub side: Option<String>,
    #[serde(default)]
    pub order_type: Option<String>,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub qty: Option<String>,
    #[serde(default)]
    pub cum_exec_qty: Option<String>,
    #[serde(default)]
    pub cum_exec_value: Option<String>,
    #[serde(default)]
    pub cum_exec_fee: Option<String>,
    #[serde(default)]
    pub order_status: Option<String>,
    #[serde(default)]
    pub time_in_force: Option<String>,
    #[serde(default)]
    pub reduce_only: Option<bool>,
    #[serde(default)]
    pub close_on_trigger: Option<bool>,
    #[serde(default)]
    pub created_time: Option<String>,
    #[serde(default)]
    pub updated_time: Option<String>,
    /// Extra fields vary by order type and category.
    #[serde(flatten)]
    pub extra: Value,
}

pub type GetOpenOrdersResponse = BybitResponse<CategorizedList<OrderEntry>>;
pub type GetOrderHistoryResponse = BybitResponse<CategorizedList<OrderEntry>>;

// ── Cancel All Orders ───────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllEntry {
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub order_link_id: Option<String>,
}

pub type CancelAllOrdersResponse = BybitResponse<CategorizedList<CancelAllEntry>>;

// ── Batch Orders ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderEntry {
    pub category: String,
    pub symbol: String,
    pub order_id: String,
    pub order_link_id: String,
    #[serde(default)]
    pub create_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderResult {
    pub list: Vec<BatchOrderEntry>,
}

pub type PlaceBatchOrderResponse = BybitResponse<BatchOrderResult>;
pub type AmendBatchOrderResponse = BybitResponse<BatchOrderResult>;
pub type CancelBatchOrderResponse = BybitResponse<BatchOrderResult>;

// ── Borrow Quota ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowQuota {
    pub symbol: String,
    #[serde(default)]
    pub max_trade_qty: Option<String>,
    #[serde(default)]
    pub side: Option<String>,
    #[serde(default)]
    pub max_trade_amount: Option<String>,
    #[serde(default)]
    pub borrow_coin: Option<String>,
    /// Extra fields.
    #[serde(flatten)]
    pub extra: Value,
}

pub type GetBorrowQuotaResponse = BybitResponse<BorrowQuota>;

// ── Set DCP ─────────────────────────────────────────────────────────────

pub type SetDcpResponse = BybitResponse<Value>;

// ── Pre-check Order ─────────────────────────────────────────────────────

pub type PreCheckOrderResponse = BybitResponse<Value>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_order_response() {
        let json = r#"{
            "retCode": 0, "retMsg": "OK",
            "result": {"orderId": "abc123", "orderLinkId": "link456"},
            "retExtInfo": {}, "time": 1234567890
        }"#;
        let resp: PlaceOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.result.order_id, "abc123");
        assert_eq!(resp.result.order_link_id, "link456");
    }

    #[test]
    fn test_open_orders_response() {
        let json = r#"{
            "retCode": 0, "retMsg": "OK",
            "result": {
                "category": "linear",
                "list": [{
                    "symbol": "BTCUSDT", "orderId": "abc",
                    "side": "Buy", "orderType": "Limit",
                    "price": "30000", "qty": "0.001",
                    "orderStatus": "New"
                }],
                "nextPageCursor": ""
            },
            "retExtInfo": {}, "time": 1234567890
        }"#;
        let resp: GetOpenOrdersResponse = serde_json::from_str(json).unwrap();
        let order = &resp.result.list[0];
        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.side.as_deref(), Some("Buy"));
    }
}
