//! Rate limit endpoint request parameters — mirrors `pybit/_v5_rate_limit.py`.

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetApiRateLimitParams {
    pub list: Vec<Value>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApiRateLimitParams {
    pub uids: String,
}

// get_api_rate_limit_cap — no parameters needed.
// get_all_api_rate_limits — no parameters needed.
