//! Position endpoint paths — mirrors `pybit/position.py`.

pub const GET_POSITIONS: &str = "/v5/position/list";
pub const SET_LEVERAGE: &str = "/v5/position/set-leverage";
pub const SWITCH_MARGIN_MODE: &str = "/v5/position/switch-isolated";
pub const SET_TP_SL_MODE: &str = "/v5/position/set-tpsl-mode";
pub const SWITCH_POSITION_MODE: &str = "/v5/position/switch-mode";
pub const SET_RISK_LIMIT: &str = "/v5/position/set-risk-limit";
pub const SET_TRADING_STOP: &str = "/v5/position/trading-stop";
pub const SET_AUTO_ADD_MARGIN: &str = "/v5/position/set-auto-add-margin";
pub const ADD_MARGIN: &str = "/v5/position/add-margin";
pub const GET_EXECUTIONS: &str = "/v5/execution/list";
pub const GET_CLOSED_PNL: &str = "/v5/position/closed-pnl";
pub const MOVE_POSITION: &str = "/v5/position/move-positions";
pub const GET_MOVE_POSITION_HISTORY: &str = "/v5/position/move-history";
pub const GET_CLOSED_OPTIONS_POSITIONS: &str = "/v5/position/get-closed-positions";
pub const CONFIRM_PENDING_MMR: &str = "/v5/position/confirm-pending-mmr";

#[cfg(test)]
pub(crate) const ALL: &[&str] = &[
    GET_POSITIONS, SET_LEVERAGE, SWITCH_MARGIN_MODE, SET_TP_SL_MODE,
    SWITCH_POSITION_MODE, SET_RISK_LIMIT, SET_TRADING_STOP, SET_AUTO_ADD_MARGIN,
    ADD_MARGIN, GET_EXECUTIONS, GET_CLOSED_PNL, MOVE_POSITION,
    GET_MOVE_POSITION_HISTORY, GET_CLOSED_OPTIONS_POSITIONS, CONFIRM_PENDING_MMR,
];
