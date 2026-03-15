//! Asset endpoint paths — mirrors `pybit/asset.py`.

pub const GET_COIN_EXCHANGE_RECORDS: &str = "/v5/asset/exchange/order-record";
pub const GET_OPTION_DELIVERY_RECORD: &str = "/v5/asset/delivery-record";
pub const GET_USDC_CONTRACT_SETTLEMENT: &str = "/v5/asset/settlement-record";
pub const GET_SPOT_ASSET_INFO: &str = "/v5/asset/transfer/query-asset-info";
pub const GET_ALL_COINS_BALANCE: &str = "/v5/asset/transfer/query-account-coins-balance";
pub const GET_SINGLE_COIN_BALANCE: &str = "/v5/asset/transfer/query-account-coin-balance";
pub const GET_TRANSFERABLE_COIN: &str = "/v5/asset/transfer/query-transfer-coin-list";
pub const CREATE_INTERNAL_TRANSFER: &str = "/v5/asset/transfer/inter-transfer";
pub const GET_INTERNAL_TRANSFER_RECORDS: &str = "/v5/asset/transfer/query-inter-transfer-list";
pub const GET_SUB_UID: &str = "/v5/asset/transfer/query-sub-member-list";
pub const ENABLE_UT_FOR_SUB_UID: &str = "/v5/asset/transfer/save-transfer-sub-member";
pub const CREATE_UNIVERSAL_TRANSFER: &str = "/v5/asset/transfer/universal-transfer";
pub const GET_UNIVERSAL_TRANSFER_RECORDS: &str = "/v5/asset/transfer/query-universal-transfer-list";
pub const GET_ALLOWED_DEPOSIT_COIN_INFO: &str = "/v5/asset/deposit/query-allowed-list";
pub const SET_DEPOSIT_ACCOUNT: &str = "/v5/asset/deposit/deposit-to-account";
pub const GET_DEPOSIT_RECORDS: &str = "/v5/asset/deposit/query-record";
pub const GET_SUB_ACCOUNT_DEPOSIT_RECORDS: &str = "/v5/asset/deposit/query-sub-member-record";
pub const GET_INTERNAL_DEPOSIT_RECORDS: &str = "/v5/asset/deposit/query-internal-record";
pub const GET_MASTER_DEPOSIT_ADDRESS: &str = "/v5/asset/deposit/query-address";
pub const GET_SUB_DEPOSIT_ADDRESS: &str = "/v5/asset/deposit/query-sub-member-address";
pub const GET_COIN_INFO: &str = "/v5/asset/coin/query-info";
pub const GET_WITHDRAWAL_ADDRESS_LIST: &str = "/v5/asset/withdraw/query-address";
pub const GET_WITHDRAWAL_RECORDS: &str = "/v5/asset/withdraw/query-record";
pub const GET_WITHDRAWABLE_AMOUNT: &str = "/v5/asset/withdraw/withdrawable-amount";
pub const GET_EXCHANGE_ENTITY_LIST: &str = "/v5/asset/withdraw/vasp/list";
pub const WITHDRAW: &str = "/v5/asset/withdraw/create";
pub const CANCEL_WITHDRAWAL: &str = "/v5/asset/withdraw/cancel";
pub const GET_CONVERT_COIN_LIST: &str = "/v5/asset/exchange/query-coin-list";
pub const REQUEST_A_QUOTE: &str = "/v5/asset/exchange/quote-apply";
pub const CONFIRM_A_QUOTE: &str = "/v5/asset/exchange/convert-execute";
pub const GET_CONVERT_STATUS: &str = "/v5/asset/exchange/convert-result-query";
pub const GET_CONVERT_HISTORY: &str = "/v5/asset/exchange/query-convert-history";
pub const GET_SMALL_BALANCE_COINS: &str = "/v5/asset/covert/small-balance-list";
pub const REQUEST_A_QUOTE_SMALL_BALANCE: &str = "/v5/asset/covert/get-quote";
pub const CONFIRM_A_QUOTE_SMALL_BALANCE: &str = "/v5/asset/covert/small-balance-execute";
pub const GET_EXCHANGE_HISTORY_SMALL_BALANCE: &str = "/v5/asset/covert/small-balance-history";

#[cfg(test)]
pub(crate) const ALL: &[&str] = &[
    GET_COIN_EXCHANGE_RECORDS, GET_OPTION_DELIVERY_RECORD, GET_USDC_CONTRACT_SETTLEMENT,
    GET_SPOT_ASSET_INFO, GET_ALL_COINS_BALANCE, GET_SINGLE_COIN_BALANCE,
    GET_TRANSFERABLE_COIN, CREATE_INTERNAL_TRANSFER, GET_INTERNAL_TRANSFER_RECORDS,
    GET_SUB_UID, ENABLE_UT_FOR_SUB_UID, CREATE_UNIVERSAL_TRANSFER,
    GET_UNIVERSAL_TRANSFER_RECORDS, GET_ALLOWED_DEPOSIT_COIN_INFO, SET_DEPOSIT_ACCOUNT,
    GET_DEPOSIT_RECORDS, GET_SUB_ACCOUNT_DEPOSIT_RECORDS, GET_INTERNAL_DEPOSIT_RECORDS,
    GET_MASTER_DEPOSIT_ADDRESS, GET_SUB_DEPOSIT_ADDRESS, GET_COIN_INFO,
    GET_WITHDRAWAL_ADDRESS_LIST, GET_WITHDRAWAL_RECORDS, GET_WITHDRAWABLE_AMOUNT,
    GET_EXCHANGE_ENTITY_LIST, WITHDRAW, CANCEL_WITHDRAWAL, GET_CONVERT_COIN_LIST,
    REQUEST_A_QUOTE, CONFIRM_A_QUOTE, GET_CONVERT_STATUS, GET_CONVERT_HISTORY,
    GET_SMALL_BALANCE_COINS, REQUEST_A_QUOTE_SMALL_BALANCE,
    CONFIRM_A_QUOTE_SMALL_BALANCE, GET_EXCHANGE_HISTORY_SMALL_BALANCE,
];
