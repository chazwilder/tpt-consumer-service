use derive_more::Constructor;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use sqlx_oldapi::types::chrono::NaiveDateTime;
use serde::{Deserialize, Serialize, Deserializer, Serializer};


#[derive(sqlx_oldapi::FromRow, Debug, Clone, Deserialize, Serialize, Constructor)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(non_snake_case)]
pub struct ShippingOrder {
    pub TC_SHIPMENT_ID: i64,
    pub MA_SHIPMENT_ID: i64,
    pub D_ADDRESS: String,
    pub D_CITY: String,
    pub D_STATE_PROV: String,
    pub D_POSTAL_CODE: i32,
    pub NUM_STOPS: i32,
    #[serde(deserialize_with = "deserialize_dttm", serialize_with = "serialize_dttm")]
    pub PICKUP_START_DTTM: NaiveDateTime,
    pub PRELOAD: Option<String>,
    pub CUSTOMER_NAME: Option<String>,
    pub TRAILER_NUMBER: Option<String>,
    #[serde(deserialize_with = "deserialize_dttm", serialize_with = "serialize_dttm")]
    pub CREATED_SOURCE_DTTM: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_dttm", serialize_with = "serialize_dttm")]
    pub ACTUAL_CHECKIN_DTTM: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_dttm", serialize_with = "serialize_dttm")]
    pub YARD_ACTIVITY_DTTM: NaiveDateTime,
    pub APPOINTMENT_TIME_VARIANCE: i64,
    pub APPOINTMENT_ADHERENCE: String,
    pub APPOINTMENT_ID: i64,
    pub TRAILER_REF_ID: i64,
    pub ACTIVITY_TYPE: i32,
    pub ACTIVITY_USER: String,
    pub APPOINTMENT_STATUS: String,
    pub DOCK_DOOR: String,
    pub CARRIER: String,
}


const FORMAT: &str = "%Y-%m-%d %H:%M:%S";
fn deserialize_dttm<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
}

fn serialize_dttm<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format(FORMAT));
    serializer.serialize_str(&s)
}


/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug, Clone, Constructor)]
#[allow(non_snake_case)]
pub struct MongoShipments {
    pub CREATED_DTTM: Option<DateTime<Utc>>,
    pub CHECKIN_DTTM: Option<DateTime<Utc>>,
    pub TRAILER_INSPECTED_DTTM: Option<DateTime<Utc>>,
    pub FIRST_DROP_DTTM: Option<DateTime<Utc>>,
    pub LOADED_DTTM: Option<DateTime<Utc>>,
    pub CHECKOUT_DTTM: Option<DateTime<Utc>>,
    pub MA_SHIPMENT_ID: Option<i64>,
    pub SDM_SHIPMENT_ID: Option<i64>,
    pub TRIP_NUMBER: i64,
    pub DESTINATION: Option<String>,
    pub DOCK_DOOR: Option<String>,
    pub TRAILER_NUMBER: Option<String>,
    pub CHECK_IN_USER: Option<String>,
    pub CUSTOMER: Option<String>,
    pub CARRIER: Option<String>,
    pub APPOINTMENT_DTTM: Option<DateTime<Utc>>,
    pub APPOINTMENT_ADHERENCE: Option<String>,
    pub APPOINTMENT_TIME_VARIANCE: Option<i64>,
    pub APPOINTMENT_STATUS: Option<String>,
    pub GRANT_DETENTION: Option<bool>,
    pub SKU: Option<Vec<String>>,
    pub SKU_LOCATION_COUNT: Option<i32>,
    pub PALLET_COUNT: Option<i32>,
    pub PLANT_ASSETS: Option<PlantAssets>,
    pub LOCATIONS: Option<Vec<Location>>,
    pub AGING_LPNS: Option<Vec<AgingLPN>>,
    pub MULTISTOP_COUNT: Option<i32>,
    pub LOAD_TYPE: Option<String>,
    pub LOAD_TIME: Option<i32>,
    pub PROCESS_TIME: Option<i32>,
    pub GATE_TO_DOCK: Option<i32>,
    pub REDOCKED: Option<bool>,
    pub DOCK_TO_INSPECTED: Option<i32>,
    pub START_TO_FIRST_DROP: Option<i32>,
    pub PRODUCTION_BLOCKS: Option<i32>,
    pub LOAD_PATTERN: Option<Vec<LoadPattern>>,
    pub TIME_MACHINE: Option<TimeMachine>,
    pub APPLICATION_SETTING: Option<HashMap<String, serde_json::Value>>,
    pub CROSSDOCKING_ENABLED: Option<bool>,
    pub LOADED_LPNS: Option<Vec<LoadedLPN>>,
    pub MA_APPOINTMENT_ID: Option<i64>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Constructor)]
pub struct PlantAssets {
    DOCKS_AVAILABLE: Option<i32>,
    DOCKS_ALLOCATED: Option<i32>,
    LIVE_LOAD_COUNT: Option<i32>,
    PRELOAD_COUNT: Option<i32>,
    LGVS_IN_THE_SYSTEM: Option<i32>,
    LGVS_REMOVED: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Constructor)]
pub struct Location {
    LOCATION_ID: Option<i32>,
    LOCATION_NAME: Option<String>,
    UNITS_IN_LOCATION: Option<i32>,
    UNITS_ALLOCATABLE: Option<i32>,
    LOCATION_BLOCKED: Option<bool>,
    LOCATION_DISABLE: Option<bool>,
    POSITIONS_BLOCKED: Option<i32>,
    POSITION_DISABLED: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Constructor)]
pub struct AgingLPN {
    STOCKUNIT_ID: Option<i32>,
    SKU: Option<String>,
    LPN: Option<String>,
    ENTER_DTTM: Option<DateTime<Utc>>,
    EXPIRATION_DTTM: Option<DateTime<Utc>>,
    LOCATION_NAME: Option<String>,
    LOCATION_ID: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Constructor)]
pub struct LoadPattern {
    key: Option<i32>,
    value: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Constructor)]
pub struct TimeMachine {
    TRANSPORT_ORDERS: Option<Vec<TransportOrder>>,
    MISSIONS: Option<Vec<Mission>>,
    LOADING_STATUS: Option<Vec<LoadingStatus>>,
    LGV_METRICS: Option<LGVMetrics>,
    LPN_ACTIVITY: Option<Vec<LPNActivity>>,
    STAFFING: Option<Staffing>,
    LGV_MANAGER_MISSIONS: Option<HashMap<String, serde_json::Value>>, // Assuming it's a map of some kind
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Constructor)]
pub struct TransportOrder {
    LOG_DTTM: Option<DateTime<Utc>>,
    TRANSPORT_ORDER_ID: Option<i32>,
    TRANSPORT_ORDER_STATUS: Option<String>,
    ALLOCATED_LPN: Option<String>,
    ALLOCATED_LGV: Option<String>,
    START_DTTM: Option<DateTime<Utc>>,
    RUNNING_DTTM: Option<DateTime<Utc>>,
    COMPLETED_DTTM: Option<DateTime<Utc>>,
    HANDLING_ORDER: Option<String>,
    FROM_LOCATION: Option<String>,
    TO_LOCATION: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Constructor)]
pub struct Mission {
    LOG_DTTM: Option<DateTime<Utc>>,
    MISSION_ID: Option<i32>,
    MISSION_STATUS: Option<String>,
    HANDLING_ORDER: Option<String>,
    MISSION_REQUEST_DTTM: Option<DateTime<Utc>>,
    MISSION_RELEASED_DTTM: Option<DateTime<Utc>>,
    MISSION_ALLOCATION_DTTM: Option<DateTime<Utc>>,
    MISSION_PU_DTTM: Option<DateTime<Utc>>,
    MISSION_DROP_DTTM: Option<DateTime<Utc>>,
    MISSION_END_DTTM: Option<DateTime<Utc>>,
    DROP_POSITION: Option<i32>,
    MISSION_TIME: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Constructor)]
pub struct LoadingStatus {
    LOG_DTTM: Option<DateTime<Utc>>,
    STATUS: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Constructor)]
pub struct LGVMetrics {
    GLOBAL_LGVS: Option<GlobalLGVS>,
    ACTIVATED_LGV: Option<Vec<ActivatedLGV>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Constructor)]
pub struct GlobalLGVS {
    LOG_DTTM: Option<DateTime<Utc>>,
    LGVS_ACTIVE: Option<i32>,
    LGVS_IDLE: Option<i32>,
    LGVS_IN_ALARM: Option<i32>,
    ALARMED_LGV_COORD: Option<HashMap<i32, Coordinates>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Constructor)]
pub struct ActivatedLGV {
    LOG_DTTM: Option<DateTime<Utc>>,
    LGV: Option<i32>,
    LOADED: Option<bool>,
    TRAFFIC_BLOCKED: Option<bool>,
    IN_ALARM: Option<bool>,
    X_COORD: Option<i32>,
    Y_COORD: Option<i32>,
    ALARM_COUNT: Option<i32>,
    MISSION_ID: Option<i32>,
    TRANSPORT_ORDER_ID: Option<i32>,
    WAITING_CMD: Option<i32>,
    AUTOMATIC: Option<bool>,
    IN_PATH: Option<bool>,
    VALID_POSITION: Option<bool>,
    MOVING_FW: Option<bool>,
    MOVING_BW: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Constructor)]
pub struct Coordinates {
    x: Option<i32>,
    y: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Constructor)]
pub struct LPNActivity {
    STOCKUNIT_ID: Option<i32>,
    LPN: Option<String>,
    PALLETS_IN_FRONT: Option<i32>,
    POSITION_DISABLED: Option<bool>,
    POSITIONS_BLOCKED: Option<bool>,
    LOCATION_NAME: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Constructor)]
pub struct Staffing {
    SPRING_LOADS: Option<i32>,
    RESIN_LOADS: Option<i32>,
    HEADCOUNT: Option<i32>,
    BREAKS: Option<HashMap<String, DateTime<Utc>>>,
    TEAM_MEMBERS: Option<Vec<String>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Constructor)]
pub struct LoadedLPN {
    STOCKUNIT_ID: Option<i32>,
    SKU: Option<String>,
    LPN: Option<String>,
    LOT_NUMBER: Option<String>,
    QUANTITY: Option<i32>,
}
