mod commands;
use commands::*;

use poise::serenity_prelude as serenity;
use serde_derive::{Deserialize};

#[derive(Deserialize, Debug)]
struct Config {
    // last_fm_key: String,
    // last_fm_ua: String,
    discord_token: String,
    // banned_words: Vec<String>,
    developers: Vec<String>,
    //reddit: Reddit,
    //imgur: Imgur
}

// #[derive(Deserialize, Debug)]
// struct Reddit {
//     client_id: String,
//     client_secret: String,
//     user_agent: String
// }
//
// #[derive(Deserialize, Debug)]
// struct Imgur {
//     imgur_id: String,
//     imgur_secret: String,
//     authorization: String
// }

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
pub struct Data {}

fn check_dev(id: String) -> bool {
    let config_path = "config.json";
    let config_read = std::fs::read_to_string(&config_path);

    let config: Config = serde_json::from_str(&config_read.unwrap()).unwrap();

    if config.developers.contains(&id) {
        return true;
    }

    return false;
}


#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
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

// TODO: Figure out a way to update client status
// #[poise::command(prefix_command)]
// async fn status(ctx: Context<'_>, msg: serenity::Message) -> Result<(), Error> {
//     let mut args = msg.content.splitn(2, ' ');
//
//
//     ctx.set_activity(serenity::Activity::playing("Hello")).await;
//     Ok(())
// }

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
                // general::test() <== Uncomment this when you need it
            ],
            ..Default::default()
        })
        .token(config.discord_token)
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();
}