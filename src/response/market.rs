//! Market endpoint response types — mirrors `pybit/_v5_market.py` responses.

use serde::Deserialize;
use serde_json::Value;

use super::common::{BybitResponse, CategorizedList, SymbolList};

// ── Server Time ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub time_second: String,
    pub time_nano: String,
}

pub type ServerTimeResponse = BybitResponse<ServerTime>;

// ── Kline (OHLCV) ──────────────────────────────────────────────────────

/// Each kline entry is `[startTime, open, high, low, close, volume, turnover]`.
pub type KlineEntry = Vec<String>;

pub type KlineResponse = BybitResponse<SymbolList<KlineEntry>>;

// Also used by mark_price_kline, index_price_kline, premium_index_price_kline.
pub type MarkPriceKlineResponse = KlineResponse;
pub type IndexPriceKlineResponse = KlineResponse;
pub type PremiumIndexPriceKlineResponse = KlineResponse;

// ── Instruments Info ────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilter {
    pub min_leverage: String,
    pub max_leverage: String,
    pub leverage_step: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    pub min_price: String,
    pub max_price: String,
    pub tick_size: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    pub max_order_qty: String,
    pub min_order_qty: String,
    pub qty_step: String,
    #[serde(default)]
    pub post_only_max_order_qty: Option<String>,
    #[serde(default)]
    pub max_mkt_order_qty: Option<String>,
    #[serde(default)]
    pub min_notional_value: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskParameters {
    #[serde(default)]
    pub price_limit_ratio_x: Option<String>,
    #[serde(default)]
    pub price_limit_ratio_y: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentInfo {
    pub symbol: String,
    #[serde(default)]
    pub contract_type: Option<String>,
    pub status: String,
    pub base_coin: String,
    pub quote_coin: String,
    #[serde(default)]
    pub launch_time: Option<String>,
    #[serde(default)]
    pub delivery_time: Option<String>,
    #[serde(default)]
    pub delivery_fee_rate: Option<String>,
    #[serde(default)]
    pub price_scale: Option<String>,
    #[serde(default)]
    pub leverage_filter: Option<LeverageFilter>,
    #[serde(default)]
    pub price_filter: Option<PriceFilter>,
    #[serde(default)]
    pub lot_size_filter: Option<LotSizeFilter>,
    #[serde(default)]
    pub unified_margin_trade: Option<bool>,
    #[serde(default)]
    pub funding_interval: Option<u32>,
    #[serde(default)]
    pub settle_coin: Option<String>,
    #[serde(default)]
    pub copy_trading: Option<String>,
    #[serde(default)]
    pub upper_funding_rate: Option<String>,
    #[serde(default)]
    pub lower_funding_rate: Option<String>,
    #[serde(default)]
    pub is_pre_listing: Option<bool>,
    #[serde(default)]
    pub pre_listing_info: Option<Value>,
    #[serde(default)]
    pub risk_parameters: Option<RiskParameters>,
    /// Extra fields that may appear in different categories (spot, option, etc.)
    #[serde(flatten)]
    pub extra: Value,
}

pub type InstrumentsInfoResponse = BybitResponse<CategorizedList<InstrumentInfo>>;

// ── Orderbook ───────────────────────────────────────────────────────────

/// Each orderbook level is `[price, size]` as strings.
pub type OrderbookLevel = Vec<String>;

#[derive(Debug, Clone, Deserialize)]
pub struct Orderbook {
    pub s: String,
    pub b: Vec<OrderbookLevel>,
    pub a: Vec<OrderbookLevel>,
    pub ts: u64,
    pub u: u64,
    #[serde(default)]
    pub seq: Option<u64>,
    #[serde(default)]
    pub cts: Option<u64>,
}

pub type OrderbookResponse = BybitResponse<Orderbook>;

// ── Tickers ─────────────────────────────────────────────────────────────

/// Ticker data varies by category (linear/inverse/spot/option).
/// Using a flat struct with optional fields to cover all variants.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub symbol: String,
    #[serde(default)]
    pub last_price: Option<String>,
    #[serde(default)]
    pub index_price: Option<String>,
    #[serde(default)]
    pub mark_price: Option<String>,
    #[serde(default)]
    pub prev_price_24h: Option<String>,
    #[serde(default)]
    pub price_24h_pcnt: Option<String>,
    #[serde(default)]
    pub high_price_24h: Option<String>,
    #[serde(default)]
    pub low_price_24h: Option<String>,
    #[serde(default)]
    pub prev_price_1h: Option<String>,
    #[serde(default)]
    pub open_interest: Option<String>,
    #[serde(default)]
    pub open_interest_value: Option<String>,
    #[serde(default)]
    pub turnover_24h: Option<String>,
    #[serde(default)]
    pub volume_24h: Option<String>,
    #[serde(default)]
    pub funding_rate: Option<String>,
    #[serde(default)]
    pub next_funding_time: Option<String>,
    #[serde(default)]
    pub bid1_price: Option<String>,
    #[serde(default)]
    pub bid1_size: Option<String>,
    #[serde(default)]
    pub ask1_price: Option<String>,
    #[serde(default)]
    pub ask1_size: Option<String>,
    /// Extra fields for different categories.
    #[serde(flatten)]
    pub extra: Value,
}

pub type TickersResponse = BybitResponse<CategorizedList<Ticker>>;

// ── Funding Rate History ────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateEntry {
    pub symbol: String,
    pub funding_rate: String,
    pub funding_rate_timestamp: String,
}

pub type FundingRateHistoryResponse = BybitResponse<CategorizedList<FundingRateEntry>>;

// ── Public Trade History ────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicTrade {
    pub exec_id: String,
    pub symbol: String,
    pub price: String,
    pub size: String,
    pub side: String,
    pub time: String,
    #[serde(default)]
    pub is_block_trade: Option<bool>,
    #[serde(default, rename = "isRPITrade")]
    pub is_rpi_trade: Option<bool>,
    #[serde(default)]
    pub seq: Option<String>,
}

pub type PublicTradeHistoryResponse = BybitResponse<CategorizedList<PublicTrade>>;

// ── Open Interest ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestEntry {
    pub open_interest: String,
    pub timestamp: String,
}

pub type OpenInterestResponse = BybitResponse<SymbolList<OpenInterestEntry>>;

// ── Historical Volatility ───────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalVolatilityEntry {
    #[serde(default)]
    pub period: Option<u32>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub time: Option<String>,
}

pub type HistoricalVolatilityResponse = BybitResponse<Vec<HistoricalVolatilityEntry>>;

// ── Insurance ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceEntry {
    pub coin: String,
    pub symbols: String,
    pub balance: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceResult {
    pub updated_time: String,
    pub list: Vec<InsuranceEntry>,
}

pub type InsuranceResponse = BybitResponse<InsuranceResult>;

// ── Risk Limit ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskLimitEntry {
    pub id: u64,
    pub symbol: String,
    pub risk_limit_value: String,
    pub maintenance_margin: String,
    pub initial_margin: String,
    pub is_lowest_risk: u8,
    pub max_leverage: String,
    #[serde(default)]
    pub mm_deduction: Option<String>,
}

pub type RiskLimitResponse = BybitResponse<CategorizedList<RiskLimitEntry>>;

// ── Option Delivery Price ───────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionDeliveryPrice {
    pub symbol: String,
    #[serde(default)]
    pub delivery_price: Option<String>,
    #[serde(default)]
    pub delivery_time: Option<String>,
}

pub type OptionDeliveryPriceResponse = BybitResponse<CategorizedList<OptionDeliveryPrice>>;

// ── Long Short Ratio ────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatioEntry {
    pub symbol: String,
    #[serde(default)]
    pub buy_ratio: Option<String>,
    #[serde(default)]
    pub sell_ratio: Option<String>,
    pub timestamp: String,
}

pub type LongShortRatioResponse = BybitResponse<CategorizedList<LongShortRatioEntry>>;

// ── Price Limit ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceLimit {
    pub symbol: String,
    /// Upper price limit (buy limit).
    #[serde(default)]
    pub buy_lmt: Option<String>,
    /// Lower price limit (sell limit).
    #[serde(default)]
    pub sell_lmt: Option<String>,
    #[serde(default)]
    pub ts: Option<String>,
}

pub type PriceLimitResponse = BybitResponse<PriceLimit>;

// ── RPI Orderbook (same shape as regular Orderbook) ─────────────────────

pub type RpiOrderbookResponse = OrderbookResponse;

// ── New Delivery Price (same shape as option delivery price) ────────────

pub type NewDeliveryPriceResponse = OptionDeliveryPriceResponse;

// ── Index Price Components ──────────────────────────────────────────────

/// Index price components result — structure varies, using Value for flexibility.
pub type IndexPriceComponentsResponse = BybitResponse<Value>;

// ── ADL Alert ───────────────────────────────────────────────────────────

pub type AdlAlertResponse = BybitResponse<Value>;

// ── Fee Group Info ──────────────────────────────────────────────────────

pub type FeeGroupInfoResponse = BybitResponse<Value>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_time_deserialize() {
        let json = r#"{
            "retCode": 0, "retMsg": "OK",
            "result": {"timeSecond": "1773519049", "timeNano": "1773519049668877908"},
            "retExtInfo": {}, "time": 1773519049668
        }"#;
        let resp: ServerTimeResponse = serde_json::from_str(json).unwrap();
        assert!(resp.is_ok());
        assert_eq!(resp.result.time_second, "1773519049");
    }

    #[test]
    fn test_kline_deserialize() {
        let json = r#"{
            "retCode": 0, "retMsg": "OK",
            "result": {
                "symbol": "BTCUSDT", "category": "linear",
                "list": [["1773518400000","70663.1","70670.9","70602.3","70645.1","80.565","5691478.9074"]]
            },
            "retExtInfo": {}, "time": 1773519051568
        }"#;
        let resp: KlineResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.result.symbol.as_deref(), Some("BTCUSDT"));
        let entry = &resp.result.list[0];
        assert_eq!(entry.len(), 7);
        assert_eq!(entry[0], "1773518400000");
        assert_eq!(entry[1], "70663.1");
    }

    #[test]
    fn test_orderbook_deserialize() {
        let json = r#"{
            "retCode": 0, "retMsg": "OK",
            "result": {
                "s": "BTCUSDT",
                "b": [["70642","1.729"]],
                "a": [["70642.1","2.16"]],
                "ts": 1773519054822, "u": 1980505, "seq": 546820437702, "cts": 1773519054820
            },
            "retExtInfo": {}, "time": 1773519054848
        }"#;
        let resp: OrderbookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.result.s, "BTCUSDT");
        assert_eq!(resp.result.b[0][0], "70642");
        assert_eq!(resp.result.a[0][1], "2.16");
        assert_eq!(resp.result.ts, 1773519054822);
        assert_eq!(resp.result.seq, Some(546820437702));
    }

    #[test]
    fn test_tickers_deserialize() {
        let json = r#"{
            "retCode": 0, "retMsg": "OK",
            "result": {
                "category": "linear",
                "list": [{
                    "symbol": "BTCUSDT",
                    "lastPrice": "70642.10",
                    "bid1Price": "70642.00",
                    "ask1Price": "70642.10"
                }]
            },
            "retExtInfo": {}, "time": 1773519056337
        }"#;
        let resp: TickersResponse = serde_json::from_str(json).unwrap();
        let ticker = &resp.result.list[0];
        assert_eq!(ticker.symbol, "BTCUSDT");
        assert_eq!(ticker.last_price.as_deref(), Some("70642.10"));
    }

    #[test]
    fn test_funding_rate_deserialize() {
        let json = r#"{
            "retCode": 0, "retMsg": "OK",
            "result": {
                "category": "linear",
                "list": [{"symbol": "BTCUSDT", "fundingRate": "-0.00008727", "fundingRateTimestamp": "1773504000000"}]
            },
            "retExtInfo": {}, "time": 1773519058923
        }"#;
        let resp: FundingRateHistoryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.result.list[0].funding_rate, "-0.00008727");
    }

    #[test]
    fn test_public_trade_deserialize() {
        let json = r#"{
            "retCode": 0, "retMsg": "OK",
            "result": {
                "category": "linear",
                "list": [{
                    "execId": "abc", "symbol": "BTCUSDT", "price": "70642.10",
                    "size": "0.002", "side": "Buy", "time": "1773519053427",
                    "isBlockTrade": false, "isRPITrade": false, "seq": "546820436127"
                }]
            },
            "retExtInfo": {}, "time": 1773519060470
        }"#;
        let resp: PublicTradeHistoryResponse = serde_json::from_str(json).unwrap();
        let trade = &resp.result.list[0];
        assert_eq!(trade.side, "Buy");
        assert_eq!(trade.is_block_trade, Some(false));
    }

    #[test]
    fn test_insurance_deserialize() {
        let json = r#"{
            "retCode": 0, "retMsg": "OK",
            "result": {
                "updatedTime": "1773519000000",
                "list": [{"coin": "BTC", "symbols": "BTCUSD", "balance": "5082.44", "value": "359298822.96"}]
            },
            "retExtInfo": {}, "time": 1773519065116
        }"#;
        let resp: InsuranceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.result.list[0].coin, "BTC");
        assert_eq!(resp.result.updated_time, "1773519000000");
    }

    #[test]
    fn test_risk_limit_deserialize() {
        let json = r#"{
            "retCode": 0, "retMsg": "OK",
            "result": {
                "category": "linear",
                "list": [{"id": 1, "symbol": "BTCUSDT", "riskLimitValue": "2000000",
                          "maintenanceMargin": "0.005", "initialMargin": "0.01",
                          "isLowestRisk": 1, "maxLeverage": "100.00", "mmDeduction": ""}]
            },
            "retExtInfo": {}, "time": 1773519067234
        }"#;
        let resp: RiskLimitResponse = serde_json::from_str(json).unwrap();
        let entry = &resp.result.list[0];
        assert_eq!(entry.id, 1);
        assert_eq!(entry.is_lowest_risk, 1);
        assert_eq!(entry.max_leverage, "100.00");
    }
}
