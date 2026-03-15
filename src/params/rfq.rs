//! RFQ (Request for Quote) endpoint request parameters — mirrors `pybit/_v5_rfq.py`.

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRfqParams {
    pub base_coin: String,
    pub rfq_type: String,
    pub legs: Vec<Value>,
}

// get_rfq_config — no parameters needed.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelRfqParams {
    pub rfq_id: String,
}

// cancel_all_rfq — no parameters needed.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuoteParams {
    pub rfq_id: String,
    pub legs: Vec<Value>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteQuoteParams {
    pub quote_id: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelQuoteParams {
    pub quote_id: String,
}

// cancel_all_quotes — no parameters needed.
// get_rfq_realtime — no parameters needed.
// get_rfq_list — no parameters needed.
// get_quote_realtime — no parameters needed.
// get_quote_list — no parameters needed.
// get_trade_list — no parameters needed.
// get_public_trades — no parameters needed.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptOtherQuoteParams {
    pub quote_id: String,
}
