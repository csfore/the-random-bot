//! Description: Helper functions to reduce function clutter in files

use mongodb::{
    bson::doc,
    options::FindOptions
};

use futures::stream::TryStreamExt;
use serde_derive::{Serialize, Deserialize};
use crate::database::connect;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    developers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Dev {
    name: String,
    id: String,
}

pub fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

//pub fn check_dev(id: String) -> bool {
//    /*
//        Checks the config.json file to ensure a user is a developer to ensure they don't run privileged commands
//    */
//
//    let config_path = "config.json";
//    let config_read = std::fs::read_to_string(&config_path);
//
//    let config: Config = serde_json::from_str(&config_read.unwrap()).unwrap();
//
//    if config.developers.contains(&String::from(id)) {
//        return true;
//    }
//
//    return false;
//}

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
        println!("Dev {} tried to register commands", developers.name);
    }

    if is_dev {
        Ok(true)
    } else {
        Ok(false)
    }
}