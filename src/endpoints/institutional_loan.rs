//! Institutional loan endpoint paths — mirrors `pybit/institutional_loan.py`.

pub const GET_PRODUCT_INFO: &str = "/v5/ins-loan/product-infos";
pub const GET_MARGIN_COIN_INFO: &str = "/v5/ins-loan/ensure-tokens-convert";
pub const GET_LOAN_ORDERS: &str = "/v5/ins-loan/loan-order";
pub const GET_REPAYMENT_ORDERS: &str = "/v5/ins-loan/repaid-history";
pub const GET_LTV: &str = "/v5/ins-loan/ltv-convert";
pub const BIND_OR_UNBIND_UID: &str = "/v5/ins-loan/association-uid";
pub const REPAY_LOAN: &str = "/v5/ins-loan/repay-loan";

#[cfg(test)]
pub(crate) const ALL: &[&str] = &[
    GET_PRODUCT_INFO, GET_MARGIN_COIN_INFO, GET_LOAN_ORDERS, GET_REPAYMENT_ORDERS,
    GET_LTV, BIND_OR_UNBIND_UID, REPAY_LOAN,
];
