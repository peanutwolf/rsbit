//! RFQ endpoint paths — mirrors `pybit/rfq.py`.

pub const CREATE_RFQ: &str = "/v5/rfq/create-rfq";
pub const GET_CONFIG: &str = "/v5/rfq/config";
pub const CANCEL_RFQ: &str = "/v5/rfq/cancel-rfq";
pub const CANCEL_ALL_RFQ: &str = "/v5/rfq/cancel-all-rfq";
pub const CREATE_QUOTE: &str = "/v5/rfq/create-quote";
pub const EXECUTE_QUOTE: &str = "/v5/rfq/execute-quote";
pub const CANCEL_QUOTE: &str = "/v5/rfq/cancel-quote";
pub const CANCEL_ALL_QUOTES: &str = "/v5/rfq/cancel-all-quotes";
pub const GET_RFQ_REALTIME: &str = "/v5/rfq/rfq-realtime";
pub const GET_RFQ_LIST: &str = "/v5/rfq/rfq-list";
pub const GET_QUOTE_REALTIME: &str = "/v5/rfq/quote-realtime";
pub const GET_QUOTE_LIST: &str = "/v5/rfq/quote-list";
pub const GET_TRADE_LIST: &str = "/v5/rfq/trade-list";
pub const GET_PUBLIC_TRADES: &str = "/v5/rfq/public-trades";
pub const ACCEPT_OTHER_QUOTE: &str = "/v5/rfq/accept-other-quote";

#[cfg(test)]
pub(crate) const ALL: &[&str] = &[
    CREATE_RFQ, GET_CONFIG, CANCEL_RFQ, CANCEL_ALL_RFQ, CREATE_QUOTE, EXECUTE_QUOTE,
    CANCEL_QUOTE, CANCEL_ALL_QUOTES, GET_RFQ_REALTIME, GET_RFQ_LIST, GET_QUOTE_REALTIME,
    GET_QUOTE_LIST, GET_TRADE_LIST, GET_PUBLIC_TRADES, ACCEPT_OTHER_QUOTE,
];
