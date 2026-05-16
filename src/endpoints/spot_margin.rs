//! Spot margin trade endpoint paths — mirrors `pybit/spot_margin_trade.py`.

pub const TOGGLE_MARGIN_TRADE: &str = "/v5/spot-margin-trade/switch-mode";
pub const SET_LEVERAGE: &str = "/v5/spot-margin-trade/set-leverage";
pub const VIP_MARGIN_DATA: &str = "/v5/spot-margin-trade/data";
pub const STATUS_AND_LEVERAGE: &str = "/v5/spot-margin-trade/state";
pub const HISTORICAL_INTEREST: &str = "/v5/spot-margin-trade/interest-rate-history";
pub const GET_TIERED_COLLATERAL_RATIO: &str = "/v5/spot-margin-trade/collateral";
pub const GET_AUTO_REPAY_MODE: &str = "/v5/spot-margin-trade/get-auto-repay-mode";
pub const SET_AUTO_REPAY_MODE: &str = "/v5/spot-margin-trade/set-auto-repay-mode";
pub const NORMAL_GET_VIP_MARGIN_DATA: &str = "/v5/spot-cross-margin-trade/data";
pub const NORMAL_GET_MARGIN_COIN_INFO: &str = "/v5/spot-cross-margin-trade/pledge-token";
pub const NORMAL_GET_BORROWABLE_COIN_INFO: &str = "/v5/spot-cross-margin-trade/borrow-token";
pub const NORMAL_GET_INTEREST_QUOTA: &str = "/v5/spot-cross-margin-trade/loan-info";
pub const NORMAL_GET_LOAN_ACCOUNT_INFO: &str = "/v5/spot-cross-margin-trade/account";
pub const NORMAL_BORROW: &str = "/v5/spot-cross-margin-trade/loan";
pub const NORMAL_REPAY: &str = "/v5/spot-cross-margin-trade/repay";
pub const NORMAL_GET_BORROW_ORDER_DETAIL: &str = "/v5/spot-cross-margin-trade/orders";
pub const NORMAL_GET_REPAYMENT_ORDER_DETAIL: &str = "/v5/spot-cross-margin-trade/repay-history";
pub const NORMAL_TOGGLE_MARGIN_TRADE: &str = "/v5/spot-cross-margin-trade/switch";
pub const GET_MAX_BORROWABLE: &str = "/v5/spot-margin-trade/max-borrowable";
pub const GET_POSITION_TIERS: &str = "/v5/spot-margin-trade/position-tiers";
pub const GET_COIN_STATE: &str = "/v5/spot-margin-trade/coinstate";
pub const GET_REPAYMENT_AVAILABLE_AMOUNT: &str = "/v5/spot-margin-trade/repayment-available-amount";
pub const GET_CURRENCY_DATA: &str = "/v5/spot-margin-trade/currency-data";

#[cfg(test)]
pub(crate) const ALL: &[&str] = &[
    TOGGLE_MARGIN_TRADE, SET_LEVERAGE, VIP_MARGIN_DATA, STATUS_AND_LEVERAGE,
    HISTORICAL_INTEREST, GET_TIERED_COLLATERAL_RATIO, GET_AUTO_REPAY_MODE,
    SET_AUTO_REPAY_MODE, NORMAL_GET_VIP_MARGIN_DATA, NORMAL_GET_MARGIN_COIN_INFO,
    NORMAL_GET_BORROWABLE_COIN_INFO, NORMAL_GET_INTEREST_QUOTA,
    NORMAL_GET_LOAN_ACCOUNT_INFO, NORMAL_BORROW, NORMAL_REPAY,
    NORMAL_GET_BORROW_ORDER_DETAIL, NORMAL_GET_REPAYMENT_ORDER_DETAIL,
    NORMAL_TOGGLE_MARGIN_TRADE, GET_MAX_BORROWABLE, GET_POSITION_TIERS,
    GET_COIN_STATE, GET_REPAYMENT_AVAILABLE_AMOUNT, GET_CURRENCY_DATA,
];
