//! Fiat endpoint response types.

use serde_json::Value;

use super::common::BybitResponse;

pub type FiatCoinListResponse = BybitResponse<Value>;
pub type FiatReferencePriceResponse = BybitResponse<Value>;
pub type RequestFiatQuoteResponse = BybitResponse<Value>;
pub type ExecuteFiatTradeResponse = BybitResponse<Value>;
pub type QueryFiatTradeResponse = BybitResponse<Value>;
pub type FiatTradeHistoryResponse = BybitResponse<Value>;
pub type FiatBalanceResponse = BybitResponse<Value>;
