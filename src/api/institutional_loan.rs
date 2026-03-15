//! Institutional loan HTTP endpoint methods — mirrors `pybit/_v5_institutional_loan.py`.

use crate::endpoints::institutional_loan;
use crate::error::Result;
use crate::http::{ApiResponse, HttpManager, Method};
use crate::params::institutional_loan::*;

use super::params_to_map;

impl HttpManager {
    pub async fn get_product_info(&self, params: GetProductInfoParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, institutional_loan::GET_PRODUCT_INFO, Some(params_to_map(&params)), false).await
    }

    pub async fn get_margin_coin_info(&self, params: GetMarginCoinInfoParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, institutional_loan::GET_MARGIN_COIN_INFO, Some(params_to_map(&params)), false).await
    }

    pub async fn get_loan_orders(&self, params: GetLoanOrdersParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, institutional_loan::GET_LOAN_ORDERS, Some(params_to_map(&params)), true).await
    }

    pub async fn get_repayment_orders(&self, params: GetRepaymentInfoParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, institutional_loan::GET_REPAYMENT_ORDERS, Some(params_to_map(&params)), true).await
    }

    pub async fn get_ltv(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, institutional_loan::GET_LTV, None, true).await
    }

    pub async fn bind_or_unbind_uid(&self, params: BindOrUnbindUidParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, institutional_loan::BIND_OR_UNBIND_UID, Some(params_to_map(&params)), true).await
    }

    pub async fn repay_loan(&self, params: RepayLoanParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, institutional_loan::REPAY_LOAN, Some(params_to_map(&params)), true).await
    }
}
