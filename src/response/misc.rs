//! Miscellaneous endpoint response types.

use serde_json::Value;

use super::common::BybitResponse;

pub type AnnouncementResponse = BybitResponse<Value>;
pub type DemoTradingFundsResponse = BybitResponse<Value>;
pub type SystemStatusResponse = BybitResponse<Value>;
