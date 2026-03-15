//! Crypto loan endpoint request parameters — mirrors `pybit/_v5_crypto_loan.py`.

use serde::Serialize;
use serde_json::Value;

// ── Legacy Crypto Loan ──────────────────────────────────────────────────

// get_collateral_coins — no parameters needed.
// get_borrowable_coins — no parameters needed.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountBorrowableOrCollateralizableLimitParams {
    pub loan_currency: String,
    pub collateral_currency: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowCryptoLoanParams {
    pub loan_currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_amount: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepayCryptoLoanParams {
    pub order_id: String,
    pub amount: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUnpaidLoansParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoanRepaymentHistoryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repay_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCompletedLoanHistoryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxAllowedCollateralReductionAmountParams {
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustCollateralAmountParams {
    pub currency: String,
    pub amount: String,
    pub direction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCryptoLoanLtvAdjustmentHistoryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

// ── New Crypto Loan ─────────────────────────────────────────────────────

// get_borrowable_coins_new — no parameters needed.
// get_collateral_coins_new — no parameters needed.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxCollateralReductionNewParams {
    pub currency: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustCollateralAmountNewParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLtvAdjustmentHistoryNewParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

// get_position_new — no parameters needed.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowFlexibleCryptoLoanParams {
    pub loan_currency: String,
    pub loan_amount: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepayFlexibleCryptoLoanParams {
    pub loan_currency: String,
    pub loan_amount: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralRepaymentFlexibleParams {
    pub loan_currency: String,
    pub collateral_coin: String,
    pub amount: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFlexibleLoansParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBorrowingHistoryFlexibleParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRepaymentHistoryFlexibleParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

// ── Fixed Crypto Loan ───────────────────────────────────────────────────
// These endpoints use generic kwargs in Python. We use Value for flexibility.

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLendingMarketFixedParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBorrowingMarketFixedParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Generic params for fixed crypto loan POST endpoints that lack documented parameters.
/// Users pass arbitrary key-value pairs matching the Bybit API docs.
#[derive(Debug, Default, Serialize)]
pub struct FixedCryptoLoanGenericParams {
    #[serde(flatten)]
    pub params: serde_json::Map<String, Value>,
}

/// Generic params for fixed crypto loan GET endpoints that lack documented parameters.
#[derive(Debug, Default, Serialize)]
pub struct FixedCryptoLoanGenericQueryParams {
    #[serde(flatten)]
    pub params: serde_json::Map<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrow_crypto_loan_serialization() {
        let params = BorrowCryptoLoanParams {
            loan_currency: "USDT".into(),
            loan_amount: Some("1000".into()),
            collateral_currency: Some("BTC".into()),
            ..Default::default()
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["loanCurrency"], "USDT");
        assert_eq!(json["loanAmount"], "1000");
        assert_eq!(json["collateralCurrency"], "BTC");
        assert!(json.get("collateralAmount").is_none());
    }

    #[test]
    fn test_generic_fixed_params() {
        let mut params = FixedCryptoLoanGenericParams::default();
        params.params.insert("orderId".into(), Value::String("123".into()));
        params.params.insert("amount".into(), Value::String("100".into()));
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["orderId"], "123");
        assert_eq!(json["amount"], "100");
    }
}
