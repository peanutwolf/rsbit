//! P2P endpoint paths — mirrors `pybit/p2p.py`.

pub const GET_ACCOUNT_INFORMATION: &str = "/v5/p2p/user/personal/info";
pub const GET_ADS_LIST: &str = "/v5/p2p/item/personal/list";
pub const GET_AD_DETAILS: &str = "/v5/p2p/item/info";
pub const UPDATE_AD: &str = "/v5/p2p/item/update";
pub const REMOVE_AD: &str = "/v5/p2p/item/cancel";
pub const GET_ORDERS: &str = "/v5/p2p/order/simplifyList";
pub const GET_PENDING_ORDERS: &str = "/v5/p2p/order/pending/simplifyList";
pub const GET_COUNTERPARTY_INFO: &str = "/v5/p2p/user/order/personal/info";
pub const GET_ORDER_DETAILS: &str = "/v5/p2p/order/info";
pub const RELEASE_ASSETS: &str = "/v5/p2p/order/finish";
pub const MARK_AS_PAID: &str = "/v5/p2p/order/pay";
pub const GET_CHAT_MESSAGES: &str = "/v5/p2p/order/message/listpage";
pub const UPLOAD_CHAT_FILE: &str = "/v5/p2p/oss/upload_file";
pub const SEND_CHAT_MESSAGE: &str = "/v5/p2p/order/message/send";
pub const POST_NEW_AD: &str = "/v5/p2p/item/create";
pub const GET_ONLINE_ADS: &str = "/v5/p2p/item/online";
pub const GET_USER_PAYMENT_TYPES: &str = "/v5/p2p/user/payment/list";
pub const QUERY_CHAT_SESSION_LIST: &str = "/v5/p2p/chat/session/list";
pub const SEND_MESSAGE: &str = "/v5/p2p/chat/message/send";
pub const GET_MESSAGE_LIST: &str = "/v5/p2p/chat/message/listpage";

#[cfg(test)]
pub(crate) const ALL: &[&str] = &[
    GET_ACCOUNT_INFORMATION,
    GET_ADS_LIST,
    GET_AD_DETAILS,
    UPDATE_AD,
    REMOVE_AD,
    GET_ORDERS,
    GET_PENDING_ORDERS,
    GET_COUNTERPARTY_INFO,
    GET_ORDER_DETAILS,
    RELEASE_ASSETS,
    MARK_AS_PAID,
    GET_CHAT_MESSAGES,
    UPLOAD_CHAT_FILE,
    SEND_CHAT_MESSAGE,
    POST_NEW_AD,
    GET_ONLINE_ADS,
    GET_USER_PAYMENT_TYPES,
    QUERY_CHAT_SESSION_LIST,
    SEND_MESSAGE,
    GET_MESSAGE_LIST,
];
