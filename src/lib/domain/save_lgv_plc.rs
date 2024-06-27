use chrono::{DateTime, FixedOffset, NaiveDateTime};
use crate::models::ILgv;
use crate::db::mssql_rch::get_connection;
use sqlx_oldapi::{QueryBuilder,Mssql};

pub async fn lgv_plc_to_mssql(lgv: ILgv) {
    let pool = get_connection().await.expect("Failed to get database connection");
    let new_dt = lgv.log_dttm.to_string().clone();
    println!("Original date-time: {:?}", new_dt);

    // Try parsing with different formats
    let dt = DateTime::parse_from_rfc3339(&new_dt)
        .or_else(|_| DateTime::parse_from_str(&new_dt, "%Y-%m-%d %H:%M:%S%.f %z"))
        .or_else(|_| DateTime::parse_from_str(&new_dt, "%Y-%m-%d %H:%M:%S%.f"))
        .expect("Failed to parse date-time");

    let formatted_dt = dt.with_timezone(&FixedOffset::east(0)).format("%Y-%m-%d %H:%M:%S");
    println!("Formatted date-time: {:?}", formatted_dt);

    let query = format!(
        "INSERT INTO [RCH-E80-REP-DB].[dbo].[LGV_PLC_LOG] (LOG_DTTM, LGV_ID, X_POS, Y_POS, RESET_NOTIFY, AUTO_MODE, LOADED, IN_SYSTEM, POSITION_VALID, REMOVE_BLOCK_REQUEST, LOCAL_MODE, END_OP_OK, MOVING_FW, MOVING_BW, WAITING_FOR_COMMAND, ON_TARGET, END_OP_FAIL, LOW_BATTERY_ALARM, AGV_ALARM, LOW_BATTERY_WARNING)
        VALUES ('{}', {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
        formatted_dt,
        lgv.lgv_id.unwrap_or_default(),
        lgv.x_pos.unwrap_or_default(),
        lgv.y_pos.unwrap_or_default(),
        if lgv.reset_notify.unwrap_or_default() { 1 } else { 0 },
        if lgv.auto_mode.unwrap_or_default() { 1 } else { 0 },
        if lgv.loaded.unwrap_or_default() { 1 } else { 0 },
        if lgv.in_system.unwrap_or_default() { 1 } else { 0 },
        if lgv.position_valid.unwrap_or_default() { 1 } else { 0 },
        if lgv.remove_block_request.unwrap_or_default() { 1 } else { 0 },
        if lgv.local_mode.unwrap_or_default() { 1 } else { 0 },
        if lgv.end_op_ok.unwrap_or_default() { 1 } else { 0 },
        if lgv.moving_fw.unwrap_or_default() { 1 } else { 0 },
        if lgv.moving_bw.unwrap_or_default() { 1 } else { 0 },
        if lgv.waiting_for_command.unwrap_or_default() { 1 } else { 0 },
        if lgv.on_target.unwrap_or_default() { 1 } else { 0 },
        if lgv.end_op_fail.unwrap_or_default() { 1 } else { 0 },
        if lgv.low_battery_alarm.unwrap_or_default() { 1 } else { 0 },
        if lgv.agv_alarm.unwrap_or_default() { 1 } else { 0 },
        if lgv.low_battery_warning.unwrap_or_default() { 1 } else { 0 }
    );
    println!("QueryBuilder: {:?}", &query);
    sqlx_oldapi::query(&query)
        .execute(&pool)
        .await
        .map_err(|e| println!("Error executing query: {:?}", e))
        .ok();
    }