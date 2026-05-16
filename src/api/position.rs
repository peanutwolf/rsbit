//! Position management HTTP endpoint methods — mirrors `pybit/_v5_position.py`.
//!
//! All methods require authentication (auth = true).

use crate::endpoints::position;
use crate::error::Result;
use crate::http::{ApiResponse, HttpManager, Method};
use crate::params::position::*;

use super::params_to_map;

impl HttpManager {
    // ── Get Positions ──────────────────────────────────────────────

    /// Query real-time position data, such as position size, cumulative realizedPNL.
    pub async fn get_positions(
        &self,
        params: GetPositionsParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            position::GET_POSITIONS,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Set Leverage ───────────────────────────────────────────────

    /// Set the leverage.
    pub async fn set_leverage(
        &self,
        params: SetLeverageParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            position::SET_LEVERAGE,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Switch Cross/Isolated Margin Mode ──────────────────────────

    /// Select cross margin mode or isolated margin mode.
    pub async fn switch_margin_mode(
        &self,
        params: SwitchMarginModeParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            position::SWITCH_MARGIN_MODE,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Set TP/SL Mode ─────────────────────────────────────────────

    /// Set TP/SL mode to Full or Partial.
    pub async fn set_tp_sl_mode(
        &self,
        params: SetTpSlModeParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            position::SET_TP_SL_MODE,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Switch Position Mode ───────────────────────────────────────

    /// Switch between one-way mode and hedge mode.
    pub async fn switch_position_mode(
        &self,
        params: SwitchPositionModeParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            position::SWITCH_POSITION_MODE,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Set Risk Limit ─────────────────────────────────────────────

    /// Set the risk limit of a single position.
    pub async fn set_risk_limit(
        &self,
        params: SetRiskLimitParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            position::SET_RISK_LIMIT,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Set Trading Stop ───────────────────────────────────────────

    /// Set the take profit, stop loss or trailing stop for the position.
    pub async fn set_trading_stop(
        &self,
        params: SetTradingStopParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            position::SET_TRADING_STOP,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Set Auto Add Margin ────────────────────────────────────────

    /// Turn on/off auto-add-margin for isolated margin position.
    pub async fn set_auto_add_margin(
        &self,
        params: SetAutoAddMarginParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            position::SET_AUTO_ADD_MARGIN,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Add or Reduce Margin ───────────────────────────────────────

    /// Manually add or reduce margin for isolated margin position.
    pub async fn add_or_reduce_margin(
        &self,
        params: AddOrReduceMarginParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            position::ADD_MARGIN,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Executions ─────────────────────────────────────────────

    /// Query users' execution records, sorted by execTime in descending order.
    pub async fn get_executions(
        &self,
        params: GetExecutionsParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            position::GET_EXECUTIONS,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Closed PnL ─────────────────────────────────────────────

    /// Query user's closed profit and loss records.
    pub async fn get_closed_pnl(
        &self,
        params: GetClosedPnlParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            position::GET_CLOSED_PNL,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Move Position ──────────────────────────────────────────────

    /// Move positions between sub-master, master-sub, or sub-sub UIDs.
    pub async fn move_position(
        &self,
        params: MovePositionParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            position::MOVE_POSITION,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Move Position History ──────────────────────────────────

    /// Query moved position data by master UID's API key.
    pub async fn get_move_position_history(
        &self,
        params: GetMovePositionHistoryParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            position::GET_MOVE_POSITION_HISTORY,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    // ── Get Closed Options Positions ───────────────────────────────

    /// Query user's closed options positions.
    pub async fn get_closed_options_positions(
        &self,
        params: GetClosedOptionsPositionsParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Get,
            position::GET_CLOSED_OPTIONS_POSITIONS,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }

    /// Confirm acceptance of a new risk limit after a position is liquidated.
    /// <https://bybit-exchange.github.io/docs/v5/position/confirm-mmr>
    pub async fn confirm_new_risk_limit(
        &self,
        params: ConfirmNewRiskLimitParams,
    ) -> Result<ApiResponse> {
        self.submit_request(
            Method::Post,
            position::CONFIRM_PENDING_MMR,
            Some(params_to_map(&params)),
            true,
        )
        .await
    }
}
