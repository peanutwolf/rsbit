//! Crypto loan endpoint response types.

use serde_json::Value;

use super::common::BybitResponse;

// Legacy
pub type CollateralCoinsResponse = BybitResponse<Value>;
pub type BorrowableCoinsResponse = BybitResponse<Value>;
pub type AccountBorrowableLimitResponse = BybitResponse<Value>;
pub type BorrowCryptoLoanResponse = BybitResponse<Value>;
pub type RepayCryptoLoanResponse = BybitResponse<Value>;
pub type UnpaidLoansResponse = BybitResponse<Value>;
pub type LoanRepaymentHistoryResponse = BybitResponse<Value>;
pub type CompletedLoanHistoryResponse = BybitResponse<Value>;
pub type MaxCollateralReductionResponse = BybitResponse<Value>;
pub type AdjustCollateralResponse = BybitResponse<Value>;
pub type LtvAdjustmentHistoryResponse = BybitResponse<Value>;

// New
pub type BorrowableCoinsNewResponse = BybitResponse<Value>;
pub type CollateralCoinsNewResponse = BybitResponse<Value>;
pub type MaxCollateralReductionNewResponse = BybitResponse<Value>;
pub type AdjustCollateralNewResponse = BybitResponse<Value>;
pub type LtvAdjustmentHistoryNewResponse = BybitResponse<Value>;
pub type PositionNewResponse = BybitResponse<Value>;
pub type BorrowFlexibleResponse = BybitResponse<Value>;
pub type RepayFlexibleResponse = BybitResponse<Value>;
pub type CollateralRepaymentFlexibleResponse = BybitResponse<Value>;
pub type FlexibleLoansResponse = BybitResponse<Value>;
pub type BorrowingHistoryFlexibleResponse = BybitResponse<Value>;
pub type RepaymentHistoryFlexibleResponse = BybitResponse<Value>;

// Fixed
pub type LendingMarketFixedResponse = BybitResponse<Value>;
pub type BorrowingMarketFixedResponse = BybitResponse<Value>;
pub type FixedCryptoLoanGenericResponse = BybitResponse<Value>;
