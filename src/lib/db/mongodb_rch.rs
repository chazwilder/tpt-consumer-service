use std::collections::HashMap;
use dotenvy::dotenv;
use std::env;
use log::{error, info};
use mongodb::{Client, options::ClientOptions, Database};
use mongodb::bson::{doc, Document};
use mongodb::bson::oid::ObjectId;
use mongodb::options::FindOneAndUpdateOptions;
use crate::models::imq_new_order::INewOrder;
use crate::models::ishipment_details::ILoadDetails;
use crate::models::MongoShipments;


pub async fn get_db()-> Result<Database, anyhow::Error> {
    dotenv().ok();
    let url = env::var("MONGO_DB_URL").expect("MONGO_DB_URL must be set");
    let database = env::var("MONGO_DATABASE").expect("MONGO_DATABASE");
    let client_options = ClientOptions::parse(&url).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(&database);

    return Ok(db);
}

pub async fn update_shipment(inv: Vec<ILoadDetails>, new_order: INewOrder) {
    if inv.is_empty() {
        error!("Inventory vector is empty. Cannot update shipment.");
        return;
    }
    let db = get_db().await.unwrap();
    let collection = db.collection::<MongoShipments>("shipments");
    let object_id = ObjectId::parse_str(&new_order.mongo_id).unwrap();
    let filter = doc! { "_id": object_id };

    let mut sku_data = Document::new();
    for load in &inv {
        sku_data.insert(load.SKU.clone(), doc! {
            "SKU_LOCATION_COUNT": load.SKU_LOCATION_COUNT as i32,
            "TOTAL_INVENTORY": load.TOTAL_INVENTORY as i32,
            "SKU_CROSSDOCKING_ENABLED": load.SKU_CROSSDOCKING_ENABLED as i32,
            "HOLD_HOURS": load.HOLD_HOURS as i32
        });
    };

    let mut update_doc = Document::new();
    let set_doc = doc! {
        "SDM_SHIPMENT_ID": inv[0].SDM_SHIPMENT_ID,
        "SKU": sku_data,
        "PALLET_COUNT": inv.iter().map(|load| load.PALLET_COUNT).sum::<i64>(),
        "LOAD_CROSSDOCKING_ENABLED": inv[0].LOAD_CROSSDOCKING_ENABLED
    };
    info!("{:?}", &set_doc);

    update_doc.insert("$set", set_doc);
    let options = FindOneAndUpdateOptions::builder()
        .upsert(Some(false))
        .build();
match collection.find_one_and_update(filter, update_doc, options).await {
        Ok(Some(updated_doc)) => {
            info!("Updated document: {:?}", updated_doc);
        },
        Ok(None) => {
            info!("No document found with ID: {}", new_order.mongo_id);
        },
        Err(e) =>error!("Error updating document: {}", e),
    }
}

