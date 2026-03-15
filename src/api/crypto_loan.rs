//! Crypto loan HTTP endpoint methods — mirrors `pybit/_v5_crypto_loan.py`.

use crate::endpoints::crypto_loan;
use crate::error::Result;
use crate::http::{ApiResponse, HttpManager, Method};
use crate::params::crypto_loan::*;

use super::params_to_map;

impl HttpManager {
    // ── Legacy Crypto Loan ────────────────────────────────────────

    pub async fn get_collateral_coins(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_COLLATERAL_COINS, None, false).await
    }

    pub async fn get_borrowable_coins(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_BORROWABLE_COINS, None, false).await
    }

    pub async fn get_account_borrowable_or_collateralizable_limit(
        &self,
        params: GetAccountBorrowableOrCollateralizableLimitParams,
    ) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_ACCOUNT_BORROWABLE_OR_COLLATERALIZABLE_LIMIT, Some(params_to_map(&params)), true).await
    }

    pub async fn borrow_crypto_loan(&self, params: BorrowCryptoLoanParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, crypto_loan::BORROW_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn repay_crypto_loan(&self, params: RepayCryptoLoanParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, crypto_loan::REPAY_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn get_unpaid_loans(&self, params: GetUnpaidLoansParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_UNPAID_LOANS, Some(params_to_map(&params)), true).await
    }

    pub async fn get_loan_repayment_history(&self, params: GetLoanRepaymentHistoryParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_LOAN_REPAYMENT_HISTORY, Some(params_to_map(&params)), true).await
    }

    pub async fn get_completed_loan_history(&self, params: GetCompletedLoanHistoryParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_COMPLETED_LOAN_ORDER_HISTORY, Some(params_to_map(&params)), true).await
    }

    pub async fn get_max_allowed_collateral_reduction_amount(
        &self,
        params: GetMaxAllowedCollateralReductionAmountParams,
    ) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_MAX_ALLOWED_COLLATERAL_REDUCTION_AMOUNT, Some(params_to_map(&params)), true).await
    }

    pub async fn adjust_collateral_amount(&self, params: AdjustCollateralAmountParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::ADJUST_COLLATERAL_AMOUNT, Some(params_to_map(&params)), true).await
    }

    pub async fn get_crypto_loan_ltv_adjustment_history(
        &self,
        params: GetCryptoLoanLtvAdjustmentHistoryParams,
    ) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_CRYPTO_LOAN_LTV_ADJUSTMENT_HISTORY, Some(params_to_map(&params)), true).await
    }

    // ── New Crypto Loan (Common) ──────────────────────────────────

    pub async fn get_borrowable_coins_new_crypto_loan(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_BORROWABLE_COINS_NEW_CRYPTO_LOAN, None, true).await
    }

    pub async fn get_collateral_coins_new_crypto_loan(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_COLLATERAL_COINS_NEW_CRYPTO_LOAN, None, true).await
    }

    pub async fn get_max_allowed_collateral_reduction_amount_new_crypto_loan(
        &self,
        params: GetMaxCollateralReductionNewParams,
    ) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_MAX_ALLOWED_COLLATERAL_REDUCTION_AMOUNT_NEW_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn adjust_collateral_amount_new_crypto_loan(
        &self,
        params: AdjustCollateralAmountNewParams,
    ) -> Result<ApiResponse> {
        self.submit_request(Method::Post, crypto_loan::ADJUST_COLLATERAL_AMOUNT_NEW_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn get_ltv_adjustment_history_new_crypto_loan(
        &self,
        params: GetLtvAdjustmentHistoryNewParams,
    ) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_LTV_ADJUSTMENT_HISTORY_NEW_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn get_position_new_crypto_loan(&self) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_POSITION_NEW_CRYPTO_LOAN, None, true).await
    }

    // ── Flexible Crypto Loan ──────────────────────────────────────

    pub async fn borrow_flexible_crypto_loan(&self, params: BorrowFlexibleCryptoLoanParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, crypto_loan::BORROW_FLEXIBLE_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn repay_flexible_crypto_loan(&self, params: RepayFlexibleCryptoLoanParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, crypto_loan::REPAY_FLEXIBLE_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn collateral_repayment_flexible_crypto_loan(
        &self,
        params: CollateralRepaymentFlexibleParams,
    ) -> Result<ApiResponse> {
        self.submit_request(Method::Post, crypto_loan::COLLATERAL_REPAYMENT_FLEXIBLE_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn get_flexible_loans_flexible_crypto_loan(&self, params: GetFlexibleLoansParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_FLEXIBLE_LOANS_FLEXIBLE_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn get_borrowing_history_flexible_crypto_loan(
        &self,
        params: GetBorrowingHistoryFlexibleParams,
    ) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_BORROWING_HISTORY_FLEXIBLE_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn get_repayment_history_flexible_crypto_loan(
        &self,
        params: GetRepaymentHistoryFlexibleParams,
    ) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_REPAYMENT_HISTORY_FLEXIBLE_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    // ── Fixed Crypto Loan ─────────────────────────────────────────

    pub async fn get_lending_market_fixed_crypto_loan(&self, params: GetLendingMarketFixedParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_LENDING_MARKET_FIXED_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn get_borrowing_market_fixed_crypto_loan(&self, params: GetBorrowingMarketFixedParams) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_BORROWING_MARKET_FIXED_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn borrow_fixed_crypto_loan(&self, params: FixedCryptoLoanGenericParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, crypto_loan::BORROW_FIXED_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn renew_fixed_crypto_loan(&self, params: FixedCryptoLoanGenericParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, crypto_loan::RENEW_FIXED_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn create_lending_order_fixed_crypto_loan(&self, params: FixedCryptoLoanGenericParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, crypto_loan::CREATE_LENDING_ORDER_FIXED_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn create_borrowing_order_fixed_crypto_loan(&self, params: FixedCryptoLoanGenericParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, crypto_loan::CREATE_BORROWING_ORDER_FIXED_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn cancel_lending_order_fixed_crypto_loan(&self, params: FixedCryptoLoanGenericParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, crypto_loan::CANCEL_LENDING_ORDER_FIXED_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn get_borrowing_contract_info_fixed_crypto_loan(
        &self,
        params: FixedCryptoLoanGenericQueryParams,
    ) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_BORROWING_CONTRACT_INFO_FIXED_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn get_lending_contract_info_fixed_crypto_loan(
        &self,
        params: FixedCryptoLoanGenericQueryParams,
    ) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_LENDING_CONTRACT_INFO_FIXED_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn get_borrowing_orders_fixed_crypto_loan(
        &self,
        params: FixedCryptoLoanGenericQueryParams,
    ) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_BORROWING_ORDERS_FIXED_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn get_renewal_orders_fixed_crypto_loan(
        &self,
        params: FixedCryptoLoanGenericQueryParams,
    ) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_RENEWAL_ORDERS_FIXED_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn get_lending_orders_fixed_crypto_loan(
        &self,
        params: FixedCryptoLoanGenericQueryParams,
    ) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_LENDING_ORDERS_FIXED_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn repay_fixed_crypto_loan(&self, params: FixedCryptoLoanGenericParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, crypto_loan::REPAY_FIXED_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn collateral_repayment_fixed_crypto_loan(&self, params: FixedCryptoLoanGenericParams) -> Result<ApiResponse> {
        self.submit_request(Method::Post, crypto_loan::COLLATERAL_REPAYMENT_FIXED_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }

    pub async fn get_repayment_history_fixed_crypto_loan(
        &self,
        params: FixedCryptoLoanGenericQueryParams,
    ) -> Result<ApiResponse> {
        self.submit_request(Method::Get, crypto_loan::GET_REPAYMENT_HISTORY_FIXED_CRYPTO_LOAN, Some(params_to_map(&params)), true).await
    }
}
