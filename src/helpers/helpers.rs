//! Description: Helper functions to reduce function clutter in files

use mongodb::{bson::doc, options::FindOptions};
use crate::database;


use crate::database::connect;
use futures::stream::TryStreamExt;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    token: Option<String>,
    developers: Option<Vec<String>>,
    key: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct Dev {
    name: String,
    id: String,
}

pub fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

pub async fn check_dev(id: &str) -> mongodb::error::Result<bool> {
    /*
        Currently we're using a DB call everytime someone runs `register` but we will eventually add capabilities to have a DB connection up
        constantly to reduce latency. Will probably involve implementations and structs.
    */

    // Get a handle to a database.
    let db = connect::init().await.unwrap();

    //collection.insert_many(devs, None).await?;
    let typed_collection = db.collection::<Dev>("developers");
    let filter = doc! { "id": id };
    let find_options = FindOptions::builder().sort(doc! { "name": 1 }).build();
    let mut cursor = typed_collection.find(filter, find_options).await?;
    let mut is_dev = false;

    // Iterate over the results of the cursor.
    while let Some(developers) = cursor.try_next().await? {
        is_dev = true;
        warn!("{} ran a dev command", developers.name);
    }

    if is_dev {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn get_token() -> mongodb::error::Result<String> {
    let db = database::connect::init().await.unwrap();
    let typed_collection = db.collection::<Config>("config");
    let filter = doc! { "name": "Beta Token" };
    let find_options = FindOptions::builder().sort(doc! { "name": 1 }).build();
    let mut cursor = typed_collection.find(filter, find_options).await?;
    let mut token = String::new();
    // Iterate over the results of the cursor.
    while let Some(config) = cursor.try_next().await? {
        token = config.token.unwrap();
    }
    Ok(token)
}

pub async fn get_ow() -> mongodb::error::Result<String> {
    let db = database::connect::init().await.unwrap();
    let typed_collection = db.collection::<Config>("config");
    let filter = doc! { "name": "OW" };
    let find_options = FindOptions::builder().sort(doc! { "name": 1 }).build();
    let mut cursor = typed_collection.find(filter, find_options).await?;
    let mut token = String::new();
    // Iterate over the results of the cursor.
    while let Some(config) = cursor.try_next().await? {
        token = config.key.unwrap();
    }
    Ok(token)
}