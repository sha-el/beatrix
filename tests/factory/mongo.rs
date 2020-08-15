use beatrix_core::{
    bson::oid::ObjectId,
    mongodb::{Client, Database},
};
use beatrix_macro::MongoModel;

use serde::{Deserialize, Serialize};

pub async fn db() -> Database {
    let db_host = std::env::var("DB_HOST").expect("No DB_HOST url set");
    let db_name = std::env::var("DB_NAME").expect("NO DB_NAME set");

    let client = Client::with_uri_str(&db_host)
        .await
        .unwrap();
    client.database(&db_name)
}

pub async fn setup() {
    dotenv::from_filename(".env.test").ok();
    let db = db().await;

    for collection_name in db.list_collection_names(None).await.unwrap() {
        db.collection(&collection_name).drop(None).await.unwrap();
    }
}

#[derive(MongoModel, Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Customer {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub age: i32,
}
