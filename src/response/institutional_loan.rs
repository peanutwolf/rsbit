//! Institutional loan endpoint response types.

use serde_json::Value;

use super::common::BybitResponse;

pub type ProductInfoResponse = BybitResponse<Value>;
pub type MarginCoinInfoResponse = BybitResponse<Value>;
pub type LoanOrdersResponse = BybitResponse<Value>;
pub type RepaymentInfoResponse = BybitResponse<Value>;
pub type LtvResponse = BybitResponse<Value>;
pub type BindOrUnbindUidResponse = BybitResponse<Value>;
pub type RepayLoanResponse = BybitResponse<Value>;
