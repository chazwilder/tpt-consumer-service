use serde::{Deserialize, Serialize};
use derive_more::Constructor;

#[derive(Serialize, Deserialize, Constructor, Debug, Clone)]
pub struct INewOrder {
    pub mongo_id: String,
    pub trip_number: i64
}