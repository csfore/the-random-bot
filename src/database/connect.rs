//! Connecting to a database

use mongodb::{bson::doc, options::ClientOptions, Client, Database};

/// Called `init()` because this will be used to connect to the db then stay connected
pub async fn init() -> mongodb::error::Result<Database> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    // Manually set an option
    client_options.app_name = Some("The Random Bot".to_string());
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    // List the names of the databases in that cluster
    // Get a handle to a database.
    let db = client.database("randb");
    Ok(db)
}
