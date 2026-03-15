//! Market endpoint paths — mirrors `pybit/market.py`.

pub const GET_SERVER_TIME: &str = "/v5/market/time";
pub const GET_KLINE: &str = "/v5/market/kline";
pub const GET_MARK_PRICE_KLINE: &str = "/v5/market/mark-price-kline";
pub const GET_INDEX_PRICE_KLINE: &str = "/v5/market/index-price-kline";
pub const GET_PREMIUM_INDEX_PRICE_KLINE: &str = "/v5/market/premium-index-price-kline";
pub const GET_INSTRUMENTS_INFO: &str = "/v5/market/instruments-info";
pub const GET_ORDERBOOK: &str = "/v5/market/orderbook";
pub const GET_TICKERS: &str = "/v5/market/tickers";
pub const GET_FUNDING_RATE_HISTORY: &str = "/v5/market/funding/history";
pub const GET_PUBLIC_TRADING_HISTORY: &str = "/v5/market/recent-trade";
pub const GET_OPEN_INTEREST: &str = "/v5/market/open-interest";
pub const GET_HISTORICAL_VOLATILITY: &str = "/v5/market/historical-volatility";
pub const GET_INSURANCE: &str = "/v5/market/insurance";
pub const GET_RISK_LIMIT: &str = "/v5/market/risk-limit";
pub const GET_OPTION_DELIVERY_PRICE: &str = "/v5/market/delivery-price";
pub const GET_LONG_SHORT_RATIO: &str = "/v5/market/account-ratio";
pub const GET_PRICE_LIMIT: &str = "/v5/market/price-limit";
pub const GET_RPI_ORDERBOOK: &str = "/v5/market/rpi_orderbook";
pub const GET_NEW_DELIVERY_PRICE: &str = "/v5/market/new-delivery-price";
pub const GET_INDEX_PRICE_COMPONENTS: &str = "/v5/market/index-price-components";
pub const GET_ADL_ALERT: &str = "/v5/market/adlAlert";
pub const GET_FEE_GROUP_INFO: &str = "/v5/market/fee-group-info";

#[cfg(test)]
pub(crate) const ALL: &[&str] = &[
    GET_SERVER_TIME, GET_KLINE, GET_MARK_PRICE_KLINE, GET_INDEX_PRICE_KLINE,
    GET_PREMIUM_INDEX_PRICE_KLINE, GET_INSTRUMENTS_INFO, GET_ORDERBOOK, GET_TICKERS,
    GET_FUNDING_RATE_HISTORY, GET_PUBLIC_TRADING_HISTORY, GET_OPEN_INTEREST,
    GET_HISTORICAL_VOLATILITY, GET_INSURANCE, GET_RISK_LIMIT, GET_OPTION_DELIVERY_PRICE,
    GET_LONG_SHORT_RATIO, GET_PRICE_LIMIT, GET_RPI_ORDERBOOK, GET_NEW_DELIVERY_PRICE,
    GET_INDEX_PRICE_COMPONENTS, GET_ADL_ALERT, GET_FEE_GROUP_INFO,
];
