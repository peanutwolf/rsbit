//! Broker endpoint paths — mirrors `pybit/broker.py`.

pub const GET_BROKER_EARNINGS: &str = "/v5/broker/earning-record";
pub const GET_EXCHANGE_BROKER_EARNINGS: &str = "/v5/broker/earnings-info";
pub const GET_EXCHANGE_BROKER_ACCOUNT_INFO: &str = "/v5/broker/account-info";
pub const GET_SUBACCOUNT_DEPOSIT_RECORDS: &str = "/v5/broker/asset/query-sub-member-deposit-record";
pub const GET_VOUCHER_SPEC: &str = "/v5/broker/award/info";
pub const ISSUE_VOUCHER: &str = "/v5/broker/award/distribute-award";
pub const GET_ISSUED_VOUCHER: &str = "/v5/broker/award/distribution-record";
pub const GET_BROKER_ALL_RATE_LIMITS: &str = "/v5/broker/apilimit/query-all";
pub const GET_BROKER_RATE_LIMIT_CAP: &str = "/v5/broker/apilimit/query-cap";
pub const SET_BROKER_RATE_LIMIT: &str = "/v5/broker/apilimit/set";

#[cfg(test)]
pub(crate) const ALL: &[&str] = &[
    GET_BROKER_EARNINGS, GET_EXCHANGE_BROKER_EARNINGS, GET_EXCHANGE_BROKER_ACCOUNT_INFO,
    GET_SUBACCOUNT_DEPOSIT_RECORDS, GET_VOUCHER_SPEC, ISSUE_VOUCHER, GET_ISSUED_VOUCHER,
    GET_BROKER_ALL_RATE_LIMITS, GET_BROKER_RATE_LIMIT_CAP, SET_BROKER_RATE_LIMIT,
];
