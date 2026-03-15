//! Spot leverage token HTTP endpoint methods — mirrors `pybit/_v5_spot_leverage_token.py`.

use crate::endpoints::spot_leverage;
use crate::error::Result;
use crate::http::{ApiResponse, HttpManager, Method};
use crate::params::spot_leverage::*;

use super::params_to_map;

impl HttpManager {
    pub async fn get_leveraged_token_info(&self, params: GetLeveragedTokenInfoParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spot_leverage::GET_LEVERAGED_TOKEN_INFO, Some(params_to_map(&params)), false).await
    }

    pub async fn get_leveraged_token_market(&self, params: GetLeveragedTokenMarketParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spot_leverage::GET_LEVERAGED_TOKEN_MARKET, Some(params_to_map(&params)), false).await
    }

    pub async fn purchase_leveraged_token(&self, params: PurchaseLeveragedTokenParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, spot_leverage::PURCHASE, Some(params_to_map(&params)), true).await
    }

    pub async fn redeem_leveraged_token(&self, params: RedeemLeveragedTokenParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, spot_leverage::REDEEM, Some(params_to_map(&params)), true).await
    }

    pub async fn get_purchase_redemption_records(&self, params: GetPurchaseRedemptionRecordsParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, spot_leverage::GET_PURCHASE_REDEMPTION_RECORDS, Some(params_to_map(&params)), true).await
    }
}
