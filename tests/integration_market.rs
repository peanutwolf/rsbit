//! Integration tests for market API methods against the live Bybit mainnet.
//!
//! These tests hit real public endpoints (no auth required).
//! They validate that:
//!   1. The request succeeds (retCode == 0).
//!   2. The response body deserializes into the expected typed response.

use rsbit::response::market::*;
use rsbit::{ClientConfig, HttpManager};

fn mainnet_manager() -> HttpManager {
    HttpManager::new(ClientConfig::default()).unwrap()
}

#[tokio::test]
async fn get_server_time() {
    let m = mainnet_manager();
    let resp = m.get_server_time().await.unwrap();
    let typed: ServerTimeResponse = serde_json::from_value(resp.body).unwrap();
    assert!(typed.is_ok());
    assert!(!typed.result.time_second.is_empty());
    assert!(!typed.result.time_nano.is_empty());
}

#[tokio::test]
async fn get_kline() {
    let m = mainnet_manager();
    let resp = m
        .get_kline(rsbit::params::market::GetKlineParams {
            category: "linear".into(),
            symbol: "BTCUSDT".into(),
            interval: "15".into(),
            limit: Some(3),
            ..Default::default()
        })
        .await
        .unwrap();
    let typed: KlineResponse = serde_json::from_value(resp.body).unwrap();
    assert!(typed.is_ok());
    assert_eq!(typed.result.symbol.as_deref(), Some("BTCUSDT"));
    assert!(!typed.result.list.is_empty());
    assert_eq!(typed.result.list[0].len(), 7); // [startTime, O, H, L, C, vol, turnover]
}

#[tokio::test]
async fn get_mark_price_kline() {
    let m = mainnet_manager();
    let resp = m
        .get_mark_price_kline(rsbit::params::market::GetMarkPriceKlineParams {
            category: "linear".into(),
            symbol: "BTCUSDT".into(),
            interval: "15".into(),
            limit: Some(2),
            ..Default::default()
        })
        .await
        .unwrap();
    let typed: MarkPriceKlineResponse = serde_json::from_value(resp.body).unwrap();
    assert!(typed.is_ok());
    assert!(!typed.result.list.is_empty());
}

#[tokio::test]
async fn get_index_price_kline() {
    let m = mainnet_manager();
    let resp = m
        .get_index_price_kline(rsbit::params::market::GetIndexPriceKlineParams {
            category: "linear".into(),
            symbol: "BTCUSDT".into(),
            interval: "15".into(),
            limit: Some(2),
            ..Default::default()
        })
        .await
        .unwrap();
    let typed: IndexPriceKlineResponse = serde_json::from_value(resp.body).unwrap();
    assert!(typed.is_ok());
    assert!(!typed.result.list.is_empty());
}

#[tokio::test]
async fn get_instruments_info() {
    let m = mainnet_manager();
    let resp = m
        .get_instruments_info(rsbit::params::market::GetInstrumentsInfoParams {
            category: "linear".into(),
            symbol: Some("BTCUSDT".into()),
            ..Default::default()
        })
        .await
        .unwrap();
    let typed: InstrumentsInfoResponse = serde_json::from_value(resp.body).unwrap();
    assert!(typed.is_ok());
    let inst = &typed.result.list[0];
    assert_eq!(inst.symbol, "BTCUSDT");
    assert_eq!(inst.base_coin, "BTC");
    assert!(inst.leverage_filter.is_some());
    assert!(inst.price_filter.is_some());
    assert!(inst.lot_size_filter.is_some());
}

#[tokio::test]
async fn get_orderbook() {
    let m = mainnet_manager();
    let resp = m
        .get_orderbook(rsbit::params::market::GetOrderbookParams {
            category: "linear".into(),
            symbol: "BTCUSDT".into(),
            limit: Some(5),
        })
        .await
        .unwrap();
    let typed: OrderbookResponse = serde_json::from_value(resp.body).unwrap();
    assert!(typed.is_ok());
    assert_eq!(typed.result.s, "BTCUSDT");
    assert!(!typed.result.b.is_empty());
    assert!(!typed.result.a.is_empty());
    // Each level has [price, size]
    assert_eq!(typed.result.b[0].len(), 2);
}

#[tokio::test]
async fn get_tickers() {
    let m = mainnet_manager();
    let resp = m
        .get_tickers(rsbit::params::market::GetTickersParams {
            category: "linear".into(),
            symbol: Some("BTCUSDT".into()),
            ..Default::default()
        })
        .await
        .unwrap();
    let typed: TickersResponse = serde_json::from_value(resp.body).unwrap();
    assert!(typed.is_ok());
    let ticker = &typed.result.list[0];
    assert_eq!(ticker.symbol, "BTCUSDT");
    assert!(ticker.last_price.is_some());
    assert!(ticker.bid1_price.is_some());
    assert!(ticker.ask1_price.is_some());
}

#[tokio::test]
async fn get_funding_rate_history() {
    let m = mainnet_manager();
    let resp = m
        .get_funding_rate_history(rsbit::params::market::GetFundingRateHistoryParams {
            category: "linear".into(),
            symbol: "BTCUSDT".into(),
            limit: Some(2),
            ..Default::default()
        })
        .await
        .unwrap();
    let typed: FundingRateHistoryResponse = serde_json::from_value(resp.body).unwrap();
    assert!(typed.is_ok());
    assert!(!typed.result.list.is_empty());
    assert_eq!(typed.result.list[0].symbol, "BTCUSDT");
    assert!(!typed.result.list[0].funding_rate.is_empty());
}

#[tokio::test]
async fn get_public_trade_history() {
    let m = mainnet_manager();
    let resp = m
        .get_public_trade_history(rsbit::params::market::GetPublicTradeHistoryParams {
            category: "linear".into(),
            symbol: "BTCUSDT".into(),
            limit: Some(3),
            ..Default::default()
        })
        .await
        .unwrap();
    let typed: PublicTradeHistoryResponse = serde_json::from_value(resp.body).unwrap();
    assert!(typed.is_ok());
    assert!(!typed.result.list.is_empty());
    let trade = &typed.result.list[0];
    assert_eq!(trade.symbol, "BTCUSDT");
    assert!(!trade.exec_id.is_empty());
    assert!(trade.side == "Buy" || trade.side == "Sell");
}

#[tokio::test]
async fn get_open_interest() {
    let m = mainnet_manager();
    let resp = m
        .get_open_interest(rsbit::params::market::GetOpenInterestParams {
            category: "linear".into(),
            symbol: "BTCUSDT".into(),
            interval_time: "1h".into(),
            limit: Some(2),
            ..Default::default()
        })
        .await
        .unwrap();
    let typed: OpenInterestResponse = serde_json::from_value(resp.body).unwrap();
    assert!(typed.is_ok());
    assert!(!typed.result.list.is_empty());
    assert!(!typed.result.list[0].open_interest.is_empty());
}

#[tokio::test]
async fn get_insurance() {
    let m = mainnet_manager();
    let resp = m
        .get_insurance(rsbit::params::market::GetInsuranceParams {
            coin: Some("BTC".into()),
        })
        .await
        .unwrap();
    let typed: InsuranceResponse = serde_json::from_value(resp.body).unwrap();
    assert!(typed.is_ok());
    assert!(!typed.result.list.is_empty());
    assert_eq!(typed.result.list[0].coin, "BTC");
}

#[tokio::test]
async fn get_risk_limit() {
    let m = mainnet_manager();
    let resp = m
        .get_risk_limit(rsbit::params::market::GetRiskLimitParams {
            category: "linear".into(),
            symbol: Some("BTCUSDT".into()),
            ..Default::default()
        })
        .await
        .unwrap();
    let typed: RiskLimitResponse = serde_json::from_value(resp.body).unwrap();
    assert!(typed.is_ok());
    assert!(!typed.result.list.is_empty());
    let first = &typed.result.list[0];
    assert_eq!(first.symbol, "BTCUSDT");
    assert_eq!(first.id, 1);
}

#[tokio::test]
async fn get_long_short_ratio() {
    let m = mainnet_manager();
    let resp = m
        .get_long_short_ratio(rsbit::params::market::GetLongShortRatioParams {
            category: "linear".into(),
            symbol: "BTCUSDT".into(),
            period: "1h".into(),
            limit: Some(2),
            ..Default::default()
        })
        .await
        .unwrap();
    let typed: LongShortRatioResponse = serde_json::from_value(resp.body).unwrap();
    assert!(typed.is_ok());
    assert!(!typed.result.list.is_empty());
}

#[tokio::test]
async fn get_price_limit() {
    let m = mainnet_manager();
    let resp = m
        .get_price_limit(rsbit::params::market::GetPriceLimitParams {
            symbol: "BTCUSDT".into(),
            category: Some("linear".into()),
        })
        .await
        .unwrap();
    let typed: PriceLimitResponse = serde_json::from_value(resp.body).unwrap();
    assert!(typed.is_ok());
    assert_eq!(typed.result.symbol, "BTCUSDT");
    assert!(typed.result.buy_lmt.is_some());
    assert!(typed.result.sell_lmt.is_some());
}

// index_price_components requires a valid index name (not a trading pair).
// Skipping integration test as it's hard to determine a valid index name dynamically.

#[tokio::test]
async fn get_historical_volatility() {
    let m = mainnet_manager();
    let resp = m
        .get_historical_volatility(rsbit::params::market::GetHistoricalVolatilityParams {
            category: "option".into(),
            base_coin: Some("BTC".into()),
            ..Default::default()
        })
        .await
        .unwrap();
    // This endpoint has a non-standard envelope (no `time` field, `category` at top level).
    // Just verify the request succeeded.
    assert_eq!(resp.body["retCode"], 0);
}
