//! RFQ (Request for Quote) HTTP endpoint methods — mirrors `pybit/_v5_rfq.py`.

use crate::endpoints::rfq;
use crate::error::Result;
use crate::http::{ApiResponse, HttpManager, Method};
use crate::params::rfq::*;

use super::params_to_map;

impl HttpManager {
    pub async fn create_rfq(&self, params: CreateRfqParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, rfq::CREATE_RFQ, Some(params_to_map(&params)), true).await
    }

    pub async fn get_rfq_config(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, rfq::GET_CONFIG, None, true).await
    }

    pub async fn cancel_rfq(&self, params: CancelRfqParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, rfq::CANCEL_RFQ, Some(params_to_map(&params)), true).await
    }

    pub async fn cancel_all_rfq(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Post, rfq::CANCEL_ALL_RFQ, None, true).await
    }

    pub async fn create_quote(&self, params: CreateQuoteParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, rfq::CREATE_QUOTE, Some(params_to_map(&params)), true).await
    }

    pub async fn execute_quote(&self, params: ExecuteQuoteParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, rfq::EXECUTE_QUOTE, Some(params_to_map(&params)), true).await
    }

    pub async fn cancel_quote(&self, params: CancelQuoteParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, rfq::CANCEL_QUOTE, Some(params_to_map(&params)), true).await
    }

    pub async fn cancel_all_quotes(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Post, rfq::CANCEL_ALL_QUOTES, None, true).await
    }

    pub async fn get_rfq_realtime(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, rfq::GET_RFQ_REALTIME, None, true).await
    }

    pub async fn get_rfq_list(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, rfq::GET_RFQ_LIST, None, true).await
    }

    pub async fn get_quote_realtime(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, rfq::GET_QUOTE_REALTIME, None, true).await
    }

    pub async fn get_quote_list(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, rfq::GET_QUOTE_LIST, None, true).await
    }

    pub async fn get_trade_list(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, rfq::GET_TRADE_LIST, None, true).await
    }

    pub async fn get_public_trades(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, rfq::GET_PUBLIC_TRADES, None, false).await
    }

    pub async fn accept_other_quote(&self, params: AcceptOtherQuoteParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, rfq::ACCEPT_OTHER_QUOTE, Some(params_to_map(&params)), true).await
    }
}
