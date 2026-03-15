//! Spot leverage token endpoint paths — mirrors `pybit/spot_leverage_token.py`.

pub const GET_LEVERAGED_TOKEN_INFO: &str = "/v5/spot-lever-token/info";
pub const GET_LEVERAGED_TOKEN_MARKET: &str = "/v5/spot-lever-token/reference";
pub const PURCHASE: &str = "/v5/spot-lever-token/purchase";
pub const REDEEM: &str = "/v5/spot-lever-token/redeem";
pub const GET_PURCHASE_REDEMPTION_RECORDS: &str = "/v5/spot-lever-token/order-record";

#[cfg(test)]
pub(crate) const ALL: &[&str] = &[
    GET_LEVERAGED_TOKEN_INFO, GET_LEVERAGED_TOKEN_MARKET, PURCHASE, REDEEM,
    GET_PURCHASE_REDEMPTION_RECORDS,
];
