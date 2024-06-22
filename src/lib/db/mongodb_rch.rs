
use dotenvy::dotenv;
use std::env;
use mongodb::{Client, options::ClientOptions, Database};



pub async fn get_db()-> Result<Database, anyhow::Error> {
    dotenv().ok();
    let url = env::var("MONGO_DB_URL").expect("MONGO_DB_URL must be set");
    let database = env::var("MONGO_DATABASE").expect("MONGO_DATABASE");
    let client_options = ClientOptions::parse(&url).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(&database);

    return Ok(db);
}
