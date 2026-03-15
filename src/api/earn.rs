//! Earn HTTP endpoint methods — mirrors `pybit/_v5_earn.py`.

use crate::endpoints::earn;
use crate::error::Result;
use crate::http::{ApiResponse, HttpManager, Method};
use crate::params::earn::*;

use super::params_to_map;

impl HttpManager {
    pub async fn get_earn_product_info(&self, params: GetEarnProductInfoParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, earn::GET_EARN_PRODUCT_INFO, Some(params_to_map(&params)), false).await
    }

    pub async fn stake_or_redeem(&self, params: StakeOrRedeemParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, earn::STAKE_OR_REDEEM, Some(params_to_map(&params)), true).await
    }

    pub async fn get_stake_or_redemption_history(&self, params: GetStakeOrRedemptionHistoryParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, earn::GET_STAKE_OR_REDEMPTION_HISTORY, Some(params_to_map(&params)), true).await
    }

    pub async fn get_staked_position(&self, params: GetStakedPositionParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, earn::GET_STAKED_POSITION, Some(params_to_map(&params)), true).await
    }

    pub async fn get_yield(&self, params: GetYieldParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, earn::GET_YIELD, Some(params_to_map(&params)), true).await
    }

    pub async fn get_hourly_yield(&self, params: GetHourlyYieldParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, earn::GET_HOURLY_YIELD, Some(params_to_map(&params)), true).await
    }
}
