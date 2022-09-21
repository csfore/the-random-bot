//! Written By: Christopher Fore, David Horine, and Marshall Pearson
//! Written On: 2022-08-29
//! License: AGPLv3
//! Description: Originally written in Python using discord.py, we decided to rewrite the bot into
//!              Rust using serenity-rs and poise. More detail later.


mod commands;
use commands::*;
mod main_tests;
mod events;

// use serenity::model::gateway::Activity;
// use serenity::model::user::OnlineStatus;
use poise::serenity_prelude;
use serde_derive::{Serialize, Deserialize};
use mongodb::{
    bson::{
        doc,
    },
};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    discord_token_beta: String,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
pub struct Data {}

#[tokio::main]
async fn main() {


    let config_path = "config.json";
    let config_read = std::fs::read_to_string(&config_path);

    let config: Config = serde_json::from_str(&config_read.unwrap()).unwrap();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            // If you get a red line here on the last parenthesis, ignore it
            commands: vec![
                dev::register(),
                general::say(),
                general::age(),
                general::ask(),
                rand_info::num(),
                rand_info::fibonacci(),
                rand_info::wikipedia(),
                rand_info::word(),
                rand_info::fact(),
                rand_info::youtube(),
                animals::dog(),
                animals::fox(),
                animals::cat(),
                animals::panda(),
                animals::red_panda(),
                animals::bird(),
                animals::koala(),
                //general::test() //<== Uncomment this when you need it
            ],
            listener: |ctx, event, framework, user_data| {
                Box::pin(events::listener::event_listener(ctx, event, framework, user_data))
            },
            ..Default::default()
        })
        .token(config.discord_token_beta)
        .intents(serenity_prelude::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();
}