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
}
