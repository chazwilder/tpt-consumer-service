use serde::{Serialize, Deserialize};
use derive_more::Constructor;
use chrono::{DateTime, Local};


#[derive(Debug, Serialize, Deserialize, Clone, Constructor)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ILgv {
    pub log_dttm: DateTime<Local>,
    pub lgv_id: Option<i16>,
    pub x_pos: Option<f64>,
    pub y_pos: Option<f64>,
    pub reset_notify: Option<bool>,
    pub auto_mode: Option<bool>,
    pub loaded: Option<bool>,
    pub in_system: Option<bool>,
    pub position_valid: Option<bool>,
    pub remove_block_request: Option<bool>,
    pub local_mode: Option<bool>,
    pub end_op_ok: Option<bool>,
    pub moving_fw: Option<bool>,
    pub moving_bw: Option<bool>,
    pub waiting_for_command: Option<bool>,
    pub on_target: Option<bool>,
    pub end_op_fail: Option<bool>,
    pub low_battery_alarm: Option<bool>,
    pub agv_alarm: Option<bool>,
    pub low_battery_warning: Option<bool>,
}

impl Default for ILgv {
    fn default() -> Self {
        ILgv {
            log_dttm: Local::now(),
            lgv_id: None,
            x_pos: None,
            y_pos: None,
            reset_notify: None,
            auto_mode: None,
            loaded: None,
            in_system: None,
            position_valid: None,
            remove_block_request: None,
            local_mode: None,
            end_op_ok: None,
            moving_fw: None,
            moving_bw: None,
            waiting_for_command: None,
            on_target: None,
            end_op_fail: None,
            low_battery_alarm: None,
            agv_alarm: None,
            low_battery_warning: None,
        }
    }
}