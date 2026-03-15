//! Spread trading endpoint response types.

use serde_json::Value;

use super::common::BybitResponse;

pub type SpreadInstrumentsInfoResponse = BybitResponse<Value>;
pub type SpreadOrderbookResponse = BybitResponse<Value>;
pub type SpreadTickersResponse = BybitResponse<Value>;
pub type SpreadPublicTradeHistoryResponse = BybitResponse<Value>;
pub type SpreadPlaceOrderResponse = BybitResponse<Value>;
pub type SpreadAmendOrderResponse = BybitResponse<Value>;
pub type SpreadCancelOrderResponse = BybitResponse<Value>;
pub type SpreadCancelAllOrdersResponse = BybitResponse<Value>;
pub type SpreadOpenOrdersResponse = BybitResponse<Value>;
pub type SpreadOrderHistoryResponse = BybitResponse<Value>;
pub type SpreadTradeHistoryResponse = BybitResponse<Value>;
