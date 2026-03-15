//! Broker endpoint response types.

use serde_json::Value;

use super::common::BybitResponse;

pub type BrokerEarningsResponse = BybitResponse<Value>;
pub type ExchangeBrokerEarningsResponse = BybitResponse<Value>;
pub type ExchangeBrokerAccountInfoResponse = BybitResponse<Value>;
pub type SubaccountDepositRecordsResponse = BybitResponse<Value>;
pub type VoucherSpecResponse = BybitResponse<Value>;
pub type IssueVoucherResponse = BybitResponse<Value>;
pub type IssuedVoucherResponse = BybitResponse<Value>;
