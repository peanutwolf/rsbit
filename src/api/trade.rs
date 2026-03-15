//! Trade/order management HTTP endpoint methods — mirrors `pybit/_v5_trade.py`.
//!
//! All methods require authentication (auth = true).

use crate::endpoints::trade;
use crate::error::Result;
use crate::http::{ApiResponse, HttpManager, Method};
use crate::params::trade::*;

use super::params_to_map;

impl HttpManager {
    // ── Place Order ─────────────────────────────────────────────────

    /// Place a new order.
    pub async fn place_order(&self, params: PlaceOrderParams) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            trade::PLACE_ORDER,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Amend Order ─────────────────────────────────────────────────

    /// Amend an existing order.
    pub async fn amend_order(&self, params: AmendOrderParams) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            trade::AMEND_ORDER,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Cancel Order ────────────────────────────────────────────────

    /// Cancel an active order.
    pub async fn cancel_order(&self, params: CancelOrderParams) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            trade::CANCEL_ORDER,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Open Orders ─────────────────────────────────────────────

    /// Query unfilled or partially filled orders in real-time.
    pub async fn get_open_orders(&self, params: GetOpenOrdersParams) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            trade::GET_OPEN_ORDERS,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Cancel All Orders ───────────────────────────────────────────

    /// Cancel all active orders.
    pub async fn cancel_all_orders(
        &self,
        params: CancelAllOrdersParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            trade::CANCEL_ALL_ORDERS,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Order History ───────────────────────────────────────────

    /// Query order history (completed, cancelled, rejected).
    pub async fn get_order_history(
        &self,
        params: GetOrderHistoryParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            trade::GET_ORDER_HISTORY,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Batch Place Order ───────────────────────────────────────────

    /// Place multiple orders in a single request.
    pub async fn place_batch_order(
        &self,
        params: PlaceBatchOrderParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            trade::BATCH_PLACE_ORDER,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Batch Amend Order ───────────────────────────────────────────

    /// Amend multiple orders in a single request.
    pub async fn amend_batch_order(
        &self,
        params: AmendBatchOrderParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            trade::BATCH_AMEND_ORDER,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Batch Cancel Order ──────────────────────────────────────────

    /// Cancel multiple orders in a single request.
    pub async fn cancel_batch_order(
        &self,
        params: CancelBatchOrderParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            trade::BATCH_CANCEL_ORDER,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Borrow Quota ────────────────────────────────────────────

    /// Query the qty and amount of borrowable coins in spot account.
    pub async fn get_borrow_quota(
        &self,
        params: GetBorrowQuotaParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            trade::GET_BORROW_QUOTA,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Set DCP (Disconnect Cancel All) ─────────────────────────────

    /// Set the Disconnection Protect time window.
    pub async fn set_dcp(&self, params: SetDcpParams) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            trade::SET_DCP,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Pre-check Order ─────────────────────────────────────────────

    /// Pre-check an order before placing it.
    pub async fn pre_check_order(&self, params: PreCheckOrderParams) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            trade::PRE_CHECK_ORDER,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }
}
