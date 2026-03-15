//! Spot margin trade endpoint response types.

use serde_json::Value;

use super::common::BybitResponse;

pub type VipMarginDataResponse = BybitResponse<Value>;
pub type ToggleMarginTradeResponse = BybitResponse<Value>;
pub type SetLeverageResponse = BybitResponse<Value>;
pub type StatusAndLeverageResponse = BybitResponse<Value>;
pub type HistoricalInterestRateResponse = BybitResponse<Value>;
pub type NormalVipMarginDataResponse = BybitResponse<Value>;
pub type TieredCollateralRatioResponse = BybitResponse<Value>;
pub type AutoRepayModeResponse = BybitResponse<Value>;
pub type SetAutoRepayModeResponse = BybitResponse<Value>;
pub type NormalMarginCoinInfoResponse = BybitResponse<Value>;
pub type NormalBorrowableCoinInfoResponse = BybitResponse<Value>;
pub type NormalInterestQuotaResponse = BybitResponse<Value>;
pub type NormalLoanAccountInfoResponse = BybitResponse<Value>;
pub type NormalBorrowResponse = BybitResponse<Value>;
pub type NormalRepayResponse = BybitResponse<Value>;
pub type NormalBorrowOrderDetailResponse = BybitResponse<Value>;
pub type NormalRepaymentOrderDetailResponse = BybitResponse<Value>;
pub type NormalToggleMarginTradeResponse = BybitResponse<Value>;
pub type MaxBorrowableResponse = BybitResponse<Value>;
pub type PositionTiersResponse = BybitResponse<Value>;
pub type CoinStateResponse = BybitResponse<Value>;
pub type RepaymentAvailableAmountResponse = BybitResponse<Value>;
