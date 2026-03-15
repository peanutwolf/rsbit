//! Miscellaneous endpoint paths — mirrors `pybit/misc.py`.

pub const GET_ANNOUNCEMENT: &str = "/v5/announcements/index";
pub const REQUEST_DEMO_TRADING_FUNDS: &str = "/v5/account/demo-apply-money";
pub const GET_SYSTEM_STATUS: &str = "/v5/system/status";

#[cfg(test)]
pub(crate) const ALL: &[&str] = &[
    GET_ANNOUNCEMENT, REQUEST_DEMO_TRADING_FUNDS, GET_SYSTEM_STATUS,
];
