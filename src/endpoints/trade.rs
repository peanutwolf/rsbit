//! Trade endpoint paths — mirrors `pybit/trade.py`.

pub const PLACE_ORDER: &str = "/v5/order/create";
pub const AMEND_ORDER: &str = "/v5/order/amend";
pub const CANCEL_ORDER: &str = "/v5/order/cancel";
pub const GET_OPEN_ORDERS: &str = "/v5/order/realtime";
pub const CANCEL_ALL_ORDERS: &str = "/v5/order/cancel-all";
pub const GET_ORDER_HISTORY: &str = "/v5/order/history";
pub const BATCH_PLACE_ORDER: &str = "/v5/order/create-batch";
pub const BATCH_AMEND_ORDER: &str = "/v5/order/amend-batch";
pub const BATCH_CANCEL_ORDER: &str = "/v5/order/cancel-batch";
pub const GET_BORROW_QUOTA: &str = "/v5/order/spot-borrow-check";
pub const SET_DCP: &str = "/v5/order/disconnected-cancel-all";
pub const PRE_CHECK_ORDER: &str = "/v5/order/pre-check";

#[cfg(test)]
pub(crate) const ALL: &[&str] = &[
    PLACE_ORDER, AMEND_ORDER, CANCEL_ORDER, GET_OPEN_ORDERS, CANCEL_ALL_ORDERS,
    GET_ORDER_HISTORY, BATCH_PLACE_ORDER, BATCH_AMEND_ORDER, BATCH_CANCEL_ORDER,
    GET_BORROW_QUOTA, SET_DCP, PRE_CHECK_ORDER,
];
