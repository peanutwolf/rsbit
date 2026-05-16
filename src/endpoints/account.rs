//! Account endpoint paths — mirrors `pybit/account.py`.

pub const GET_WALLET_BALANCE: &str = "/v5/account/wallet-balance";
pub const GET_TRANSFERABLE_AMOUNT: &str = "/v5/account/withdrawal";
pub const UPGRADE_TO_UNIFIED_ACCOUNT: &str = "/v5/account/upgrade-to-uta";
pub const GET_BORROW_HISTORY: &str = "/v5/account/borrow-history";
pub const REPAY_LIABILITY: &str = "/v5/account/quick-repayment";
pub const GET_COLLATERAL_INFO: &str = "/v5/account/collateral-info";
pub const SET_COLLATERAL_COIN: &str = "/v5/account/set-collateral-switch";
pub const BATCH_SET_COLLATERAL_COIN: &str = "/v5/account/set-collateral-switch-batch";
pub const GET_COIN_GREEKS: &str = "/v5/asset/coin-greeks";
pub const GET_FEE_RATE: &str = "/v5/account/fee-rate";
pub const GET_ACCOUNT_INFO: &str = "/v5/account/info";
pub const GET_TRANSACTION_LOG: &str = "/v5/account/transaction-log";
pub const GET_CONTRACT_TRANSACTION_LOG: &str = "/v5/account/contract-transaction-log";
pub const SET_MARGIN_MODE: &str = "/v5/account/set-margin-mode";
pub const SET_MMP: &str = "/v5/account/mmp-modify";
pub const RESET_MMP: &str = "/v5/account/mmp-reset";
pub const GET_MMP_STATE: &str = "/v5/account/mmp-state";
pub const NO_CONVERT_REPAY: &str = "/v5/account/no-convert-repay";
pub const BORROW: &str = "/v5/account/borrow";
pub const GET_INSTRUMENTS_INFO: &str = "/v5/account/instruments-info";
pub const REPAY: &str = "/v5/account/repay";
pub const QUERY_DCP_INFO: &str = "/v5/account/query-dcp-info";
pub const SET_HEDGING_MODE: &str = "/v5/account/set-hedging-mode";
pub const GET_SMP_GROUP: &str = "/v5/account/smp-group";
pub const GET_USER_SETTING_CONFIG: &str = "/v5/account/user-setting-config";
pub const SET_LIMIT_PRICE_ACTION: &str = "/v5/account/set-limit-px-action";
pub const SET_DELTA_MODE: &str = "/v5/account/set-delta-mode";

#[cfg(test)]
pub(crate) const ALL: &[&str] = &[
    GET_WALLET_BALANCE, GET_TRANSFERABLE_AMOUNT, UPGRADE_TO_UNIFIED_ACCOUNT,
    GET_BORROW_HISTORY, REPAY_LIABILITY, GET_COLLATERAL_INFO, SET_COLLATERAL_COIN,
    BATCH_SET_COLLATERAL_COIN, GET_COIN_GREEKS, GET_FEE_RATE, GET_ACCOUNT_INFO,
    GET_TRANSACTION_LOG, GET_CONTRACT_TRANSACTION_LOG, SET_MARGIN_MODE, SET_MMP,
    RESET_MMP, GET_MMP_STATE, NO_CONVERT_REPAY, BORROW, GET_INSTRUMENTS_INFO,
    REPAY, QUERY_DCP_INFO, SET_HEDGING_MODE, GET_SMP_GROUP, GET_USER_SETTING_CONFIG,
    SET_LIMIT_PRICE_ACTION, SET_DELTA_MODE,
];
