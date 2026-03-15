//! Fiat HTTP endpoint methods — mirrors `pybit/_v5_fiat.py`.

use crate::endpoints::fiat;
use crate::error::Result;
use crate::http::{ApiResponse, HttpManager, Method};
use crate::params::fiat::*;

use super::params_to_map;

impl HttpManager {
    pub async fn get_fiat_coin_list(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, fiat::GET_COIN_LIST, None, true).await
    }

    pub async fn get_fiat_reference_price(&self, params: GetFiatReferencePriceParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, fiat::GET_REFERENCE_PRICE, Some(params_to_map(&params)), true).await
    }

    pub async fn request_fiat_quote(&self, params: RequestFiatQuoteParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, fiat::REQUEST_QUOTE, Some(params_to_map(&params)), true).await
    }

    pub async fn execute_fiat_trade(&self, params: ExecuteFiatTradeParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, fiat::EXECUTE_TRADE, Some(params_to_map(&params)), true).await
    }

    pub async fn query_fiat_trade(&self, params: QueryFiatTradeParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, fiat::QUERY_TRADE, Some(params_to_map(&params)), true).await
    }

    pub async fn get_fiat_trade_history(&self, params: GetFiatTradeHistoryParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, fiat::GET_TRADE_HISTORY, Some(params_to_map(&params)), true).await
    }

    pub async fn get_fiat_balance(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, fiat::GET_BALANCE, None, true).await
    }
}
