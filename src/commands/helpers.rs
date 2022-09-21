//! Description: Helper functions to reduce function clutter in files

use mongodb::{
    bson::{
        doc,
        Document
    },
    options::{
        ClientOptions,
        FindOptions
    },
    Client
};
use futures::stream::TryStreamExt;
use serde_derive::{Serialize, Deserialize};

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
    let mut client_options =
        ClientOptions::parse("mongodb://localhost:27017")
            .await?;
    // Manually set an option
    client_options.app_name = Some("The Random Bot".to_string());
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected to database successfully.");
    // List the names of the databases in that cluster
    // Get a handle to a database.
    let db = client.database("randb");

    //collection.insert_many(devs, None).await?;
    let typed_collection = db.collection::<Dev>("developers");
    let filter = doc! { "id": id };
    let find_options = FindOptions::builder().sort(doc! { "name": 1 }).build();
    let mut cursor = typed_collection.find(filter, find_options).await?;
    let mut is_dev = false;

    // Iterate over the results of the cursor.
    while let Some(developers) = cursor.try_next().await? {
        is_dev = true;
        println!("Dev ID: {}", developers.id);
    }

    if is_dev {
        Ok(true)
    } else {
        Ok(false)
    }
}