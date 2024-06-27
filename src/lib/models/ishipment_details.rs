use serde::{Deserialize, Serialize};
use derive_more::Constructor;
use sqlx_oldapi::FromRow;

#[derive(Serialize, Deserialize, Constructor, Debug, Clone, FromRow)]
#[allow(non_snake_case)]
pub struct ILoadDetails {
    SDM_SHIPMENT_ID: i64,
    PALLET_COUNT: i64,
    SKU: String,
    SKU_LOCATION_COUNT: i64,
    LOAD_CROSSDOCKING_ENABLED: bool,
    SKU_CROSSDOCKING_ENABLED: bool,
    HOLD_HOURS: i64,
    TOTAL_INVENTORY: i64,
}