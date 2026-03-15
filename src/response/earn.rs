//! Earn endpoint response types.

use serde_json::Value;

use super::common::BybitResponse;

pub type EarnProductInfoResponse = BybitResponse<Value>;
pub type StakeOrRedeemResponse = BybitResponse<Value>;
pub type StakeOrRedemptionHistoryResponse = BybitResponse<Value>;
pub type StakedPositionResponse = BybitResponse<Value>;
pub type YieldResponse = BybitResponse<Value>;
pub type HourlyYieldResponse = BybitResponse<Value>;
