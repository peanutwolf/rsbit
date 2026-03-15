//! RFQ endpoint response types.

use serde_json::Value;

use super::common::BybitResponse;

pub type CreateRfqResponse = BybitResponse<Value>;
pub type RfqConfigResponse = BybitResponse<Value>;
pub type CancelRfqResponse = BybitResponse<Value>;
pub type CancelAllRfqResponse = BybitResponse<Value>;
pub type CreateQuoteResponse = BybitResponse<Value>;
pub type ExecuteQuoteResponse = BybitResponse<Value>;
pub type CancelQuoteResponse = BybitResponse<Value>;
pub type CancelAllQuotesResponse = BybitResponse<Value>;
pub type RfqRealtimeResponse = BybitResponse<Value>;
pub type RfqListResponse = BybitResponse<Value>;
pub type QuoteRealtimeResponse = BybitResponse<Value>;
pub type QuoteListResponse = BybitResponse<Value>;
pub type RfqTradeListResponse = BybitResponse<Value>;
pub type RfqPublicTradesResponse = BybitResponse<Value>;
pub type AcceptOtherQuoteResponse = BybitResponse<Value>;
