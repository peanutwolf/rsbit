//! Rate limit endpoint paths — mirrors `pybit/rate_limit.py`.

pub const SET_API_RATE_LIMIT: &str = "/v5/apilimit/set";
pub const GET_API_RATE_LIMIT: &str = "/v5/apilimit/query";
pub const GET_API_RATE_LIMIT_CAP: &str = "/v5/apilimit/query-cap";
pub const GET_ALL_API_RATE_LIMITS: &str = "/v5/apilimit/query-all";

#[cfg(test)]
pub(crate) const ALL: &[&str] = &[
    SET_API_RATE_LIMIT, GET_API_RATE_LIMIT, GET_API_RATE_LIMIT_CAP, GET_ALL_API_RATE_LIMITS,
];
