//! Written By: Christopher Fore, David Horine, and Marshall Pearson
//! Written On: 2022-08-29
//! License: AGPLv3
//! Description: Originally written in Python using discord.py, we decided to rewrite the bot into
//!              Rust using serenity-rs and poise. More detail later.
//! Testing

mod commands;
use commands::*;
mod main_tests;

// use serenity::model::gateway::Activity;
// use serenity::model::user::OnlineStatus;
use poise::serenity_prelude;
use poise::serenity_prelude::{Activity, OnlineStatus};
use serde_derive::{Deserialize};

#[derive(Deserialize, Debug)]
struct Config {
    discord_token_beta: String,
    developers: Vec<String>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
pub struct Data {}

async fn event_listener(
    _ctx: &serenity_prelude::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    /*
        Runs an event listener using Serenity's built-in listener to set the status and presence to online
    */
    match event {
        poise::Event::Ready { data_about_bot } => {
            println!("{} is connected!", data_about_bot.user.name);

            let activity = Activity::playing("with dice");
            let status = OnlineStatus::Online;

            _ctx.set_presence(Some(activity), status).await;
        }
        _ => {}
    }

    Ok(())
}

fn check_dev(id: String) -> bool {
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


#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    /*
        Built-in poise command to register all slash commands globally or only in-guild
    */
    let author = u64::from(ctx.author().id).to_string();
    if check_dev(author) {
        println!("Commands registered.");
        poise::builtins::register_application_commands_buttons(ctx).await?;
    } else {
        println!("Commands failed to register");
        ctx.say("It seems you don't have permission to use this.").await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let config_path = "config.json";
    let config_read = std::fs::read_to_string(&config_path);

    let config: Config = serde_json::from_str(&config_read.unwrap()).unwrap();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            // If you get a red line here on the last parenthesis, ignore it
            commands: vec![
                register(),
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
                // general::test() <== Uncomment this when you need it
            ],
            listener: |ctx, event, framework, user_data| {
                Box::pin(event_listener(ctx, event, framework, user_data))
            },
            ..Default::default()
        })
        .token(config.discord_token_beta)
        .intents(serenity_prelude::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();
}