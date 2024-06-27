
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
    let db = get_db().await.unwrap();
    let collection = db.collection::<MongoShipments>("shipments");
    let object_id = ObjectId::parse_str(&new_order.mongo_id)?;
    let filter = doc! { "_id": object_id };

    let mut update_doc = Document::new();
    update_doc.insert("$set", doc! {
        "SKU": inv.iter().map(|load| load.SKU.clone()).collect::<Vec<String>>(),
        "SKU_LOCATION_COUNT": inv.iter().map(|load| load.SKU_LOCATION_COUNT).sum::<i64>(),
        "PALLET_COUNT": inv.iter().map(|load| load.PALLET_COUNT).sum::<i64>(),
        "CROSSDOCKING_ENABLED": inv.iter().any(|load| load.LOAD_CROSSDOCKING_ENABLED || load.SKU_CROSSDOCKING_ENABLED),
    });

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

