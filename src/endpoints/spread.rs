//! Spread trading endpoint paths — mirrors `pybit/spread.py`.

pub const GET_INSTRUMENTS_INFO: &str = "/v5/spread/instrument";
pub const GET_ORDERBOOK: &str = "/v5/spread/orderbook";
pub const GET_TICKERS: &str = "/v5/spread/tickers";
pub const GET_PUBLIC_TRADING_HISTORY: &str = "/v5/spread/recent-trade";
pub const PLACE_ORDER: &str = "/v5/spread/order/create";
pub const AMEND_ORDER: &str = "/v5/spread/order/amend";
pub const CANCEL_ORDER: &str = "/v5/spread/order/cancel";
pub const CANCEL_ALL_ORDERS: &str = "/v5/spread/order/cancel-all";
pub const GET_OPEN_ORDERS: &str = "/v5/spread/order/realtime";
pub const GET_ORDER_HISTORY: &str = "/v5/spread/order/history";
pub const GET_TRADE_HISTORY: &str = "/v5/spread/execution/list";

#[cfg(test)]
pub(crate) const ALL: &[&str] = &[
    GET_INSTRUMENTS_INFO, GET_ORDERBOOK, GET_TICKERS, GET_PUBLIC_TRADING_HISTORY,
    PLACE_ORDER, AMEND_ORDER, CANCEL_ORDER, CANCEL_ALL_ORDERS, GET_OPEN_ORDERS,
    GET_ORDER_HISTORY, GET_TRADE_HISTORY,
];
