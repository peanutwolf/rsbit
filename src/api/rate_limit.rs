//! Rate limit HTTP endpoint methods — mirrors `pybit/_v5_rate_limit.py`.

use crate::endpoints::rate_limit;
use crate::error::Result;
use crate::http::{ApiResponse, HttpManager, Method};
use crate::params::rate_limit::*;

use super::params_to_map;

impl HttpManager {
    pub async fn set_api_rate_limit(&self, params: SetApiRateLimitParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, rate_limit::SET_API_RATE_LIMIT, Some(params_to_map(&params)), true).await
    }

    pub async fn get_api_rate_limit(&self, params: GetApiRateLimitParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, rate_limit::GET_API_RATE_LIMIT, Some(params_to_map(&params)), true).await
    }

    pub async fn get_api_rate_limit_cap(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, rate_limit::GET_API_RATE_LIMIT_CAP, None, true).await
    }

    pub async fn get_all_api_rate_limits(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, rate_limit::GET_ALL_API_RATE_LIMITS, None, true).await
    }
}
