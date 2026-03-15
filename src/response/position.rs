//! Position endpoint response types — mirrors `pybit/_v5_position.py` responses.

use serde::Deserialize;
use serde_json::Value;

use super::common::{BybitResponse, CategorizedList};

// ── Get Positions ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionEntry {
    pub symbol: String,
    #[serde(default)]
    pub side: Option<String>,
    #[serde(default)]
    pub size: Option<String>,
    #[serde(default)]
    pub avg_price: Option<String>,
    #[serde(default)]
    pub position_value: Option<String>,
    #[serde(default)]
    pub leverage: Option<String>,
    #[serde(default)]
    pub mark_price: Option<String>,
    #[serde(default)]
    pub position_im: Option<String>,
    #[serde(default)]
    pub position_mm: Option<String>,
    #[serde(default)]
    pub unrealised_pnl: Option<String>,
    #[serde(default)]
    pub cum_realised_pnl: Option<String>,
    #[serde(default)]
    pub take_profit: Option<String>,
    #[serde(default)]
    pub stop_loss: Option<String>,
    #[serde(default)]
    pub trailing_stop: Option<String>,
    #[serde(default)]
    pub position_idx: Option<u8>,
    #[serde(default)]
    pub risk_id: Option<u64>,
    #[serde(default)]
    pub risk_limit_value: Option<String>,
    #[serde(default)]
    pub trade_mode: Option<u8>,
    #[serde(default)]
    pub created_time: Option<String>,
    #[serde(default)]
    pub updated_time: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

pub type GetPositionsResponse = BybitResponse<CategorizedList<PositionEntry>>;

// ── Executions (Trade History) ──────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionEntry {
    pub symbol: String,
    #[serde(default)]
    pub exec_id: Option<String>,
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub order_link_id: Option<String>,
    #[serde(default)]
    pub side: Option<String>,
    #[serde(default)]
    pub exec_price: Option<String>,
    #[serde(default)]
    pub exec_qty: Option<String>,
    #[serde(default)]
    pub exec_fee: Option<String>,
    #[serde(default)]
    pub exec_type: Option<String>,
    #[serde(default)]
    pub exec_time: Option<String>,
    #[serde(default)]
    pub fee_rate: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

pub type GetExecutionsResponse = BybitResponse<CategorizedList<ExecutionEntry>>;

// ── Closed PnL ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosedPnlEntry {
    pub symbol: String,
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub side: Option<String>,
    #[serde(default)]
    pub qty: Option<String>,
    #[serde(default)]
    pub order_price: Option<String>,
    #[serde(default)]
    pub order_type: Option<String>,
    #[serde(default)]
    pub exec_type: Option<String>,
    #[serde(default)]
    pub closed_size: Option<String>,
    #[serde(default)]
    pub cum_entry_value: Option<String>,
    #[serde(default)]
    pub avg_entry_price: Option<String>,
    #[serde(default)]
    pub cum_exit_value: Option<String>,
    #[serde(default)]
    pub avg_exit_price: Option<String>,
    #[serde(default)]
    pub closed_pnl: Option<String>,
    #[serde(default)]
    pub fill_count: Option<String>,
    #[serde(default)]
    pub leverage: Option<String>,
    #[serde(default)]
    pub created_time: Option<String>,
    #[serde(default)]
    pub updated_time: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

pub type GetClosedPnlResponse = BybitResponse<CategorizedList<ClosedPnlEntry>>;

// ── Move Position ───────────────────────────────────────────────────────

pub type MovePositionResponse = BybitResponse<Value>;
pub type GetMovePositionHistoryResponse = BybitResponse<Value>;

// ── Simple passthrough responses ────────────────────────────────────────

pub type SetLeverageResponse = BybitResponse<Value>;
pub type SwitchMarginModeResponse = BybitResponse<Value>;
pub type SetTpSlModeResponse = BybitResponse<Value>;
pub type SwitchPositionModeResponse = BybitResponse<Value>;
pub type SetRiskLimitResponse = BybitResponse<Value>;
pub type SetTradingStopResponse = BybitResponse<Value>;
pub type SetAutoAddMarginResponse = BybitResponse<Value>;
pub type AddOrReduceMarginResponse = BybitResponse<Value>;
pub type GetClosedOptionsPositionsResponse = BybitResponse<Value>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_deserialize() {
        let json = r#"{
            "retCode": 0, "retMsg": "OK",
            "result": {
                "category": "linear",
                "list": [{
                    "symbol": "BTCUSDT",
                    "side": "Buy",
                    "size": "0.001",
                    "avgPrice": "30000",
                    "leverage": "10",
                    "positionIdx": 0,
                    "unrealisedPnl": "5.00"
                }],
                "nextPageCursor": ""
            },
            "retExtInfo": {}, "time": 1234567890
        }"#;
        let resp: GetPositionsResponse = serde_json::from_str(json).unwrap();
        let pos = &resp.result.list[0];
        assert_eq!(pos.symbol, "BTCUSDT");
        assert_eq!(pos.side.as_deref(), Some("Buy"));
        assert_eq!(pos.leverage.as_deref(), Some("10"));
    }

    #[test]
    fn test_execution_deserialize() {
        let json = r#"{
            "retCode": 0, "retMsg": "OK",
            "result": {
                "category": "linear",
                "list": [{
                    "symbol": "BTCUSDT", "execId": "exec1",
                    "side": "Buy", "execPrice": "30000", "execQty": "0.001",
                    "execFee": "0.018", "execTime": "1234567890000"
                }],
                "nextPageCursor": ""
            },
            "retExtInfo": {}, "time": 1234567890
        }"#;
        let resp: GetExecutionsResponse = serde_json::from_str(json).unwrap();
        let exec = &resp.result.list[0];
        assert_eq!(exec.exec_price.as_deref(), Some("30000"));
    }
}
