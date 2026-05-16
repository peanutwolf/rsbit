//! Crypto loan endpoint paths — mirrors `pybit/crypto_loan.py`.

// Original crypto loan
pub const GET_COLLATERAL_COINS: &str = "/v5/crypto-loan/collateral-data";
pub const GET_BORROWABLE_COINS: &str = "/v5/crypto-loan/loanable-data";
pub const GET_ACCOUNT_BORROWABLE_OR_COLLATERALIZABLE_LIMIT: &str = "/v5/crypto-loan/borrowable-collateralisable-number";
pub const BORROW_CRYPTO_LOAN: &str = "/v5/crypto-loan/borrow";
pub const REPAY_CRYPTO_LOAN: &str = "/v5/crypto-loan/repay";
pub const GET_UNPAID_LOANS: &str = "/v5/crypto-loan/ongoing-orders";
pub const GET_LOAN_REPAYMENT_HISTORY: &str = "/v5/crypto-loan/repayment-history";
pub const GET_COMPLETED_LOAN_ORDER_HISTORY: &str = "/v5/crypto-loan/borrow-history";
pub const GET_MAX_ALLOWED_COLLATERAL_REDUCTION_AMOUNT: &str = "/v5/crypto-loan/max-collateral-amount";
pub const ADJUST_COLLATERAL_AMOUNT: &str = "/v5/crypto-loan/adjust-ltv";
pub const GET_CRYPTO_LOAN_LTV_ADJUSTMENT_HISTORY: &str = "/v5/crypto-loan/adjustment-history";

// New crypto loan (common)
pub const GET_BORROWABLE_COINS_NEW_CRYPTO_LOAN: &str = "/v5/crypto-loan-common/loanable-data";
pub const GET_COLLATERAL_COINS_NEW_CRYPTO_LOAN: &str = "/v5/crypto-loan-common/collateral-data";
pub const GET_MAX_ALLOWED_COLLATERAL_REDUCTION_AMOUNT_NEW_CRYPTO_LOAN: &str = "/v5/crypto-loan-common/max-collateral-amount";
pub const ADJUST_COLLATERAL_AMOUNT_NEW_CRYPTO_LOAN: &str = "/v5/crypto-loan-common/adjust-ltv";
pub const GET_LTV_ADJUSTMENT_HISTORY_NEW_CRYPTO_LOAN: &str = "/v5/crypto-loan-common/adjustment-history";
pub const GET_POSITION_NEW_CRYPTO_LOAN: &str = "/v5/crypto-loan-common/position";

// Flexible crypto loan
pub const BORROW_FLEXIBLE_CRYPTO_LOAN: &str = "/v5/crypto-loan-flexible/borrow";
pub const REPAY_FLEXIBLE_CRYPTO_LOAN: &str = "/v5/crypto-loan-flexible/repay";
pub const COLLATERAL_REPAYMENT_FLEXIBLE_CRYPTO_LOAN: &str = "/v5/crypto-loan-flexible/repay-collateral";
pub const GET_FLEXIBLE_LOANS_FLEXIBLE_CRYPTO_LOAN: &str = "/v5/crypto-loan-flexible/ongoing-coin";
pub const GET_BORROWING_HISTORY_FLEXIBLE_CRYPTO_LOAN: &str = "/v5/crypto-loan-flexible/borrow-history";
pub const GET_REPAYMENT_HISTORY_FLEXIBLE_CRYPTO_LOAN: &str = "/v5/crypto-loan-flexible/repayment-history";

// Fixed crypto loan
pub const GET_LENDING_MARKET_FIXED_CRYPTO_LOAN: &str = "/v5/crypto-loan-fixed/supply-order-quote";
pub const GET_BORROWING_MARKET_FIXED_CRYPTO_LOAN: &str = "/v5/crypto-loan-fixed/borrow-order-quote";
pub const BORROW_FIXED_CRYPTO_LOAN: &str = "/v5/crypto-loan-fixed/borrow";
pub const RENEW_FIXED_CRYPTO_LOAN: &str = "/v5/crypto-loan-fixed/renew";
pub const CREATE_LENDING_ORDER_FIXED_CRYPTO_LOAN: &str = "/v5/crypto-loan-fixed/supply";
pub const CREATE_BORROWING_ORDER_FIXED_CRYPTO_LOAN: &str = "/v5/crypto-loan-fixed/borrow-order-cancel";
pub const CANCEL_LENDING_ORDER_FIXED_CRYPTO_LOAN: &str = "/v5/crypto-loan-fixed/supply-order-cancel";
pub const GET_BORROWING_CONTRACT_INFO_FIXED_CRYPTO_LOAN: &str = "/v5/crypto-loan-fixed/borrow-contract-info";
pub const GET_LENDING_CONTRACT_INFO_FIXED_CRYPTO_LOAN: &str = "/v5/crypto-loan-fixed/supply-contract-info";
pub const GET_BORROWING_ORDERS_FIXED_CRYPTO_LOAN: &str = "/v5/crypto-loan-fixed/borrow-order-info";
pub const GET_RENEWAL_ORDERS_FIXED_CRYPTO_LOAN: &str = "/v5/crypto-loan-fixed/renew-info";
pub const GET_LENDING_ORDERS_FIXED_CRYPTO_LOAN: &str = "/v5/crypto-loan-fixed/supply-order-info";
pub const REPAY_FIXED_CRYPTO_LOAN: &str = "/v5/crypto-loan-fixed/fully-repay";
pub const COLLATERAL_REPAYMENT_FIXED_CRYPTO_LOAN: &str = "/v5/crypto-loan-fixed/repay-collateral";
pub const GET_REPAYMENT_HISTORY_FIXED_CRYPTO_LOAN: &str = "/v5/crypto-loan-fixed/repayment-history";
pub const GET_MAX_LOAN_AMOUNT_NEW_CRYPTO_LOAN: &str = "/v5/crypto-loan-common/max-loan";

#[cfg(test)]
pub(crate) const ALL: &[&str] = &[
    GET_COLLATERAL_COINS, GET_BORROWABLE_COINS,
    GET_ACCOUNT_BORROWABLE_OR_COLLATERALIZABLE_LIMIT, BORROW_CRYPTO_LOAN,
    REPAY_CRYPTO_LOAN, GET_UNPAID_LOANS, GET_LOAN_REPAYMENT_HISTORY,
    GET_COMPLETED_LOAN_ORDER_HISTORY, GET_MAX_ALLOWED_COLLATERAL_REDUCTION_AMOUNT,
    ADJUST_COLLATERAL_AMOUNT, GET_CRYPTO_LOAN_LTV_ADJUSTMENT_HISTORY,
    GET_BORROWABLE_COINS_NEW_CRYPTO_LOAN, GET_COLLATERAL_COINS_NEW_CRYPTO_LOAN,
    GET_MAX_ALLOWED_COLLATERAL_REDUCTION_AMOUNT_NEW_CRYPTO_LOAN,
    ADJUST_COLLATERAL_AMOUNT_NEW_CRYPTO_LOAN,
    GET_LTV_ADJUSTMENT_HISTORY_NEW_CRYPTO_LOAN, GET_POSITION_NEW_CRYPTO_LOAN,
    BORROW_FLEXIBLE_CRYPTO_LOAN, REPAY_FLEXIBLE_CRYPTO_LOAN,
    COLLATERAL_REPAYMENT_FLEXIBLE_CRYPTO_LOAN, GET_FLEXIBLE_LOANS_FLEXIBLE_CRYPTO_LOAN,
    GET_BORROWING_HISTORY_FLEXIBLE_CRYPTO_LOAN,
    GET_REPAYMENT_HISTORY_FLEXIBLE_CRYPTO_LOAN,
    GET_LENDING_MARKET_FIXED_CRYPTO_LOAN, GET_BORROWING_MARKET_FIXED_CRYPTO_LOAN,
    BORROW_FIXED_CRYPTO_LOAN, RENEW_FIXED_CRYPTO_LOAN,
    CREATE_LENDING_ORDER_FIXED_CRYPTO_LOAN,
    CREATE_BORROWING_ORDER_FIXED_CRYPTO_LOAN,
    CANCEL_LENDING_ORDER_FIXED_CRYPTO_LOAN,
    GET_BORROWING_CONTRACT_INFO_FIXED_CRYPTO_LOAN,
    GET_LENDING_CONTRACT_INFO_FIXED_CRYPTO_LOAN,
    GET_BORROWING_ORDERS_FIXED_CRYPTO_LOAN, GET_RENEWAL_ORDERS_FIXED_CRYPTO_LOAN,
    GET_LENDING_ORDERS_FIXED_CRYPTO_LOAN, REPAY_FIXED_CRYPTO_LOAN,
    COLLATERAL_REPAYMENT_FIXED_CRYPTO_LOAN,
    GET_REPAYMENT_HISTORY_FIXED_CRYPTO_LOAN,
    GET_MAX_LOAN_AMOUNT_NEW_CRYPTO_LOAN,
];
