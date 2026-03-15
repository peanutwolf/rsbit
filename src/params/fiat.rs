//! Fiat endpoint request parameters — mirrors `pybit/_v5_fiat.py`.

use serde::Serialize;

// get_fiat_coin_list — no parameters needed.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFiatReferencePriceParams {
    pub fiat_coin: String,
    pub crypto_coin: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestFiatQuoteParams {
    pub fiat_coin: String,
    pub crypto_coin: String,
    pub side: String,
    pub size: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteFiatTradeParams {
    pub quote_id: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryFiatTradeParams {
    pub order_id: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFiatTradeHistoryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

// get_fiat_balance — no parameters needed.
