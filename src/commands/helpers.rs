//! Description: Helper functions to reduce function clutter in files

use serde_derive::{Deserialize};

#[derive(Deserialize, Debug)]
struct Config {
    developers: Vec<String>,
}

pub fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

pub fn check_dev(id: String) -> bool {
    /*
        Checks the config.json file to ensure a user is a developer to ensure they don't run privileged commands
    */

    let config_path = "config.json";
    let config_read = std::fs::read_to_string(&config_path);

    let config: Config = serde_json::from_str(&config_read.unwrap()).unwrap();

    if config.developers.contains(&String::from(id)) {
        return true;
    }

    return false;
}