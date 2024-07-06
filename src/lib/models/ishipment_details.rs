use serde::{Deserialize, Serialize};
use derive_more::Constructor;
use sqlx_oldapi::FromRow;

#[derive(Serialize, Deserialize, Constructor, Debug, Clone, FromRow)]
#[allow(non_snake_case)]
pub struct ILoadDetails {
    pub SDM_SHIPMENT_ID: i64,
    pub PALLET_COUNT: i64,
    pub SKU: String,
    pub SKU_LOCATION_COUNT: Option<i64>,
    pub LOAD_CROSSDOCKING_ENABLED: bool,
    pub SKU_CROSSDOCKING_ENABLED: bool,
    pub HOLD_HOURS: i64,
    pub TOTAL_INVENTORY: i64,
}