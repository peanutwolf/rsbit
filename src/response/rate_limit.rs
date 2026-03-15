//! Rate limit endpoint response types.

use serde_json::Value;

use super::common::BybitResponse;

pub type SetApiRateLimitResponse = BybitResponse<Value>;
pub type GetApiRateLimitResponse = BybitResponse<Value>;
pub type GetApiRateLimitCapResponse = BybitResponse<Value>;
pub type GetAllApiRateLimitsResponse = BybitResponse<Value>;
