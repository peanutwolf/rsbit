//! Fiat endpoint paths — mirrors `pybit/fiat.py`.

pub const GET_COIN_LIST: &str = "/v5/fiat/query-coin-list";
pub const GET_REFERENCE_PRICE: &str = "/v5/fiat/reference-price";
pub const REQUEST_QUOTE: &str = "/v5/fiat/quote-apply";
pub const EXECUTE_TRADE: &str = "/v5/fiat/trade-execute";
pub const QUERY_TRADE: &str = "/v5/fiat/trade-query";
pub const GET_TRADE_HISTORY: &str = "/v5/fiat/query-trade-history";
pub const GET_BALANCE: &str = "/v5/fiat/balance-query";

#[cfg(test)]
pub(crate) const ALL: &[&str] = &[
    GET_COIN_LIST, GET_REFERENCE_PRICE, REQUEST_QUOTE, EXECUTE_TRADE,
    QUERY_TRADE, GET_TRADE_HISTORY, GET_BALANCE,
];
