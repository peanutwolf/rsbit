//! Earn endpoint paths — mirrors `pybit/earn.py`.

pub const GET_EARN_PRODUCT_INFO: &str = "/v5/earn/product";
pub const STAKE_OR_REDEEM: &str = "/v5/earn/place-order";
pub const GET_STAKE_OR_REDEMPTION_HISTORY: &str = "/v5/earn/order";
pub const GET_STAKED_POSITION: &str = "/v5/earn/position";
pub const GET_YIELD: &str = "/v5/earn/yield";
pub const GET_HOURLY_YIELD: &str = "/v5/earn/hourly-yield";

#[cfg(test)]
pub(crate) const ALL: &[&str] = &[
    GET_EARN_PRODUCT_INFO, STAKE_OR_REDEEM, GET_STAKE_OR_REDEMPTION_HISTORY,
    GET_STAKED_POSITION, GET_YIELD, GET_HOURLY_YIELD,
];
