//! Broker HTTP endpoint methods — mirrors `pybit/_v5_broker.py`.

use crate::endpoints::broker;
use crate::error::Result;
use crate::http::{ApiResponse, HttpManager, Method};
use crate::params::broker::*;

use super::params_to_map;

impl HttpManager {
    pub async fn get_broker_earnings(&self, params: GetBrokerEarningsParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, broker::GET_BROKER_EARNINGS, Some(params_to_map(&params)), true).await
    }

    pub async fn get_exchange_broker_earnings(&self, params: GetExchangeBrokerEarningsParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, broker::GET_EXCHANGE_BROKER_EARNINGS, Some(params_to_map(&params)), true).await
    }

    pub async fn get_exchange_broker_account_info(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, broker::GET_EXCHANGE_BROKER_ACCOUNT_INFO, None, true).await
    }

    pub async fn get_subaccount_deposit_records(&self, params: GetSubaccountDepositRecordsParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, broker::GET_SUBACCOUNT_DEPOSIT_RECORDS, Some(params_to_map(&params)), true).await
    }

    pub async fn get_voucher_spec(&self, params: GetVoucherSpecParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, broker::GET_VOUCHER_SPEC, Some(params_to_map(&params)), true).await
    }

    pub async fn issue_voucher(&self, params: IssueVoucherParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, broker::ISSUE_VOUCHER, Some(params_to_map(&params)), true).await
    }

    pub async fn get_issued_voucher(&self, params: GetIssuedVoucherParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, broker::GET_ISSUED_VOUCHER, Some(params_to_map(&params)), true).await
    }

    /// Get all broker API rate limits (institutional only).
    /// <https://bybit-exchange.github.io/docs/v5/broker/exchange-broker/rate-limit/query-all>
    pub async fn get_broker_all_rate_limits(&self, params: GetBrokerAllRateLimitsParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, broker::GET_BROKER_ALL_RATE_LIMITS, Some(params_to_map(&params)), true).await
    }

    /// Get broker API rate limit cap (institutional only).
    /// <https://bybit-exchange.github.io/docs/v5/broker/exchange-broker/rate-limit/query-cap>
    pub async fn get_broker_rate_limit_cap(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, broker::GET_BROKER_RATE_LIMIT_CAP, None, true).await
    }

    /// Set broker API rate limit (institutional only).
    /// <https://bybit-exchange.github.io/docs/v5/broker/exchange-broker/rate-limit/set>
    pub async fn set_broker_rate_limit(&self, params: SetBrokerRateLimitParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, broker::SET_BROKER_RATE_LIMIT, Some(params_to_map(&params)), true).await
    }
}
