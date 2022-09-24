//! Written By: Christopher Fore, David Horine, and Marshall Pearson
//! Written On: 2022-08-29
//! License: AGPLv3
//! Description: Originally written in Python using discord.py, we decided to rewrite the bot into
//!              Rust using serenity-rs and poise. More detail later.
#[macro_use]
extern crate log;

mod commands;
use commands::*;
mod database;
mod events;
mod main_tests;
use poise::serenity_prelude;

use env_logger::Env;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// User data, which is stored and accessible in all command invocations
pub struct Data {}

#[tokio::main]
async fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "warn")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let token = helpers::get_token().await.unwrap();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            // If you get a red line here on the last parenthesis, ignore it
            commands: vec![
                dev::register(),
                dev::servers(),
                general::say(),
                general::age(),
                general::ask(),
                rand_info::num(),
                rand_info::fibonacci(),
                rand_info::wikipedia(),
                rand_info::word(),
                rand_info::fact(),
                rand_info::youtube(),
                rand_info::reddit(),
                rand_info::weather(),
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
                Box::pin(events::listener::event_listener(
                    ctx, event, framework, user_data,
                ))
            },
            ..Default::default()
        })
        .token(token)
        .intents(serenity_prelude::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();
}
