//! Spread trading HTTP endpoint methods — mirrors `pybit/_v5_spread.py`.

use crate::endpoints::spread;
use crate::error::Result;
use crate::http::{ApiResponse, HttpManager, Method};
use crate::params::spread::*;

use super::params_to_map;

impl HttpManager {
    // ── Public Market Data ──────────────────────────────────────────

    pub async fn spread_get_instruments_info(&self, params: GetInstrumentsInfoParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spread::GET_INSTRUMENTS_INFO, Some(params_to_map(&params)), false).await
    }

    pub async fn spread_get_orderbook(&self, params: GetOrderbookParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spread::GET_ORDERBOOK, Some(params_to_map(&params)), false).await
    }

    pub async fn spread_get_tickers(&self, params: GetTickersParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spread::GET_TICKERS, Some(params_to_map(&params)), false).await
    }

    pub async fn spread_get_public_trade_history(&self, params: GetPublicTradeHistoryParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spread::GET_PUBLIC_TRADING_HISTORY, Some(params_to_map(&params)), false).await
    }

    // ── Authenticated Trading ───────────────────────────────────────

    pub async fn spread_place_order(&self, params: PlaceOrderParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, spread::PLACE_ORDER, Some(params_to_map(&params)), true).await
    }

    pub async fn spread_amend_order(&self, params: AmendOrderParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, spread::AMEND_ORDER, Some(params_to_map(&params)), true).await
    }

    pub async fn spread_cancel_order(&self, params: CancelOrderParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, spread::CANCEL_ORDER, Some(params_to_map(&params)), true).await
    }

    pub async fn spread_cancel_all_orders(&self, params: CancelAllOrdersParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, spread::CANCEL_ALL_ORDERS, Some(params_to_map(&params)), true).await
    }

    pub async fn spread_get_open_orders(&self, params: GetOpenOrdersParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spread::GET_OPEN_ORDERS, Some(params_to_map(&params)), true).await
    }

    pub async fn spread_get_order_history(&self, params: GetOrderHistoryParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spread::GET_ORDER_HISTORY, Some(params_to_map(&params)), true).await
    }

    pub async fn spread_get_trade_history(&self, params: GetTradeHistoryParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spread::GET_TRADE_HISTORY, Some(params_to_map(&params)), true).await
    }
}
