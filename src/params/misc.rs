//! Miscellaneous endpoint request parameters — mirrors `pybit/_v5_misc.py`.

use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAnnouncementParams {
    pub locale: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub announcement_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

// request_demo_trading_funds — no parameters needed.
// get_system_status — no parameters needed.
