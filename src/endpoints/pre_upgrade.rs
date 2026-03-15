//! Pre-upgrade endpoint paths — mirrors `pybit/pre_upgrade.py`.

pub const GET_PRE_UPGRADE_ORDER_HISTORY: &str = "/v5/pre-upgrade/order/history";
pub const GET_PRE_UPGRADE_TRADE_HISTORY: &str = "/v5/pre-upgrade/execution/list";
pub const GET_PRE_UPGRADE_CLOSED_PNL: &str = "/v5/pre-upgrade/position/closed-pnl";
pub const GET_PRE_UPGRADE_TRANSACTION_LOG: &str = "/v5/pre-upgrade/account/transaction-log";
pub const GET_PRE_UPGRADE_OPTION_DELIVERY_RECORD: &str = "/v5/pre-upgrade/asset/delivery-record";
pub const GET_PRE_UPGRADE_USDC_SESSION_SETTLEMENT: &str = "/v5/pre-upgrade/asset/settlement-record";

#[cfg(test)]
pub(crate) const ALL: &[&str] = &[
    GET_PRE_UPGRADE_ORDER_HISTORY, GET_PRE_UPGRADE_TRADE_HISTORY,
    GET_PRE_UPGRADE_CLOSED_PNL, GET_PRE_UPGRADE_TRANSACTION_LOG,
    GET_PRE_UPGRADE_OPTION_DELIVERY_RECORD, GET_PRE_UPGRADE_USDC_SESSION_SETTLEMENT,
];
