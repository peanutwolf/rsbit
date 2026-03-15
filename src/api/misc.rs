//! Miscellaneous HTTP endpoint methods — mirrors `pybit/_v5_misc.py`.

use crate::endpoints::misc;
use crate::error::Result;
use crate::http::{ApiResponse, HttpManager, Method};
use crate::params::misc::*;

use super::params_to_map;

impl HttpManager {
    pub async fn get_announcement(&self, params: GetAnnouncementParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, misc::GET_ANNOUNCEMENT, Some(params_to_map(&params)), false).await
    }

    pub async fn request_demo_trading_funds(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Post, misc::REQUEST_DEMO_TRADING_FUNDS, None, true).await
    }

    pub async fn get_system_status(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, misc::GET_SYSTEM_STATUS, None, false).await
    }
}
