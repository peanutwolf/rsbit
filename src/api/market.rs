//! Market data HTTP endpoint methods — mirrors `pybit/_v5_market.py`.
//!
//! All methods are public, non-authenticated GET requests.

use crate::endpoints::market;
use crate::error::Result;
use crate::http::{ApiResponse, HttpManager, Method};
use crate::params::market::*;

use super::params_to_map;

impl HttpManager {
    // ── Server Time ─────────────────────────────────────────────────

    /// Get the server time.
    pub async fn get_server_time(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, market::GET_SERVER_TIME, None, false)
            .await
    }

    // ── Kline (OHLCV) ──────────────────────────────────────────────

    /// Get kline/candlestick data.
    pub async fn get_kline(&self, params: GetKlineParams) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            market::GET_KLINE,
            Some(params_to_map(&params)),
            false,
        )
        .await
    }

    /// Get mark price kline data.
    pub async fn get_mark_price_kline(
        &self,
        params: GetMarkPriceKlineParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            market::GET_MARK_PRICE_KLINE,
            Some(params_to_map(&params)),
            false,
        )
        .await
    }

    /// Get index price kline data.
    pub async fn get_index_price_kline(
        &self,
        params: GetIndexPriceKlineParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            market::GET_INDEX_PRICE_KLINE,
            Some(params_to_map(&params)),
            false,
        )
        .await
    }

    /// Get premium index price kline data.
    pub async fn get_premium_index_price_kline(
        &self,
        params: GetPremiumIndexPriceKlineParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            market::GET_PREMIUM_INDEX_PRICE_KLINE,
            Some(params_to_map(&params)),
            false,
        )
        .await
    }

    // ── Instruments ─────────────────────────────────────────────────

    /// Get instrument specification.
    pub async fn get_instruments_info(
        &self,
        params: GetInstrumentsInfoParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            market::GET_INSTRUMENTS_INFO,
            Some(params_to_map(&params)),
            false,
        )
        .await
    }

    // ── Orderbook ───────────────────────────────────────────────────

    /// Get orderbook depth data.
    pub async fn get_orderbook(&self, params: GetOrderbookParams) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            market::GET_ORDERBOOK,
            Some(params_to_map(&params)),
            false,
        )
        .await
    }

    // ── Tickers ─────────────────────────────────────────────────────

    /// Get ticker information.
    pub async fn get_tickers(&self, params: GetTickersParams) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            market::GET_TICKERS,
            Some(params_to_map(&params)),
            false,
        )
        .await
    }

    // ── Funding Rate ────────────────────────────────────────────────

    /// Get funding rate history.
    pub async fn get_funding_rate_history(
        &self,
        params: GetFundingRateHistoryParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            market::GET_FUNDING_RATE_HISTORY,
            Some(params_to_map(&params)),
            false,
        )
        .await
    }

    // ── Public Trades ───────────────────────────────────────────────

    /// Get recent public trading history.
    pub async fn get_public_trade_history(
        &self,
        params: GetPublicTradeHistoryParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            market::GET_PUBLIC_TRADING_HISTORY,
            Some(params_to_map(&params)),
            false,
        )
        .await
    }

    // ── Open Interest ───────────────────────────────────────────────

    /// Get open interest data.
    pub async fn get_open_interest(
        &self,
        params: GetOpenInterestParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            market::GET_OPEN_INTEREST,
            Some(params_to_map(&params)),
            false,
        )
        .await
    }

    // ── Historical Volatility ───────────────────────────────────────

    /// Get historical volatility (options).
    pub async fn get_historical_volatility(
        &self,
        params: GetHistoricalVolatilityParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            market::GET_HISTORICAL_VOLATILITY,
            Some(params_to_map(&params)),
            false,
        )
        .await
    }

    // ── Insurance ───────────────────────────────────────────────────

    /// Get insurance pool data.
    pub async fn get_insurance(&self, params: GetInsuranceParams) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            market::GET_INSURANCE,
            Some(params_to_map(&params)),
            false,
        )
        .await
    }

    // ── Risk Limit ──────────────────────────────────────────────────

    /// Get risk limit information.
    pub async fn get_risk_limit(&self, params: GetRiskLimitParams) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            market::GET_RISK_LIMIT,
            Some(params_to_map(&params)),
            false,
        )
        .await
    }

    // ── Option Delivery Price ───────────────────────────────────────

    /// Get option delivery price.
    pub async fn get_option_delivery_price(
        &self,
        params: GetOptionDeliveryPriceParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            market::GET_OPTION_DELIVERY_PRICE,
            Some(params_to_map(&params)),
            false,
        )
        .await
    }

    // ── Long Short Ratio ────────────────────────────────────────────

    /// Get long/short ratio.
    pub async fn get_long_short_ratio(
        &self,
        params: GetLongShortRatioParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            market::GET_LONG_SHORT_RATIO,
            Some(params_to_map(&params)),
            false,
        )
        .await
    }

    // ── Price Limit ─────────────────────────────────────────────────

    /// Get the price limit of a symbol.
    pub async fn get_price_limit(&self, params: GetPriceLimitParams) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            market::GET_PRICE_LIMIT,
            Some(params_to_map(&params)),
            false,
        )
        .await
    }

    // ── RPI Orderbook ───────────────────────────────────────────────

    /// Get the RPI orderbook.
    pub async fn get_rpi_orderbook(
        &self,
        params: GetRpiOrderbookParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            market::GET_RPI_ORDERBOOK,
            Some(params_to_map(&params)),
            false,
        )
        .await
    }

    // ── New Delivery Price ──────────────────────────────────────────

    /// Get new delivery price.
    pub async fn get_new_delivery_price(
        &self,
        params: GetNewDeliveryPriceParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            market::GET_NEW_DELIVERY_PRICE,
            Some(params_to_map(&params)),
            false,
        )
        .await
    }

    // ── Index Price Components ──────────────────────────────────────

    /// Get index price components.
    pub async fn get_index_price_components(
        &self,
        params: GetIndexPriceComponentsParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            market::GET_INDEX_PRICE_COMPONENTS,
            Some(params_to_map(&params)),
            false,
        )
        .await
    }

    // ── ADL Alert ───────────────────────────────────────────────────

    /// Get ADL alert information.
    pub async fn get_adl_alert(&self, params: GetAdlAlertParams) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            market::GET_ADL_ALERT,
            Some(params_to_map(&params)),
            false,
        )
        .await
    }

    // ── Fee Group Info ──────────────────────────────────────────────

    /// Get fee group info.
    pub async fn get_fee_group_info(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, market::GET_FEE_GROUP_INFO, None, false)
            .await
    }
}
