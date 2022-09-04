mod commands;
use commands::*;

use poise::serenity_prelude as serenity;
use dotenv::dotenv;


type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
pub struct Data {}

// TODO: Ensure only devs can run this
#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    /// Registers the slash commands either on a global or guild level

    poise::builtins::register_application_commands_buttons(ctx).await?;
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
    dotenv().ok();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            // if you get a red line here on the last parenthesis, ignore it
            commands: vec![
                general::age(),
                register(),
                general::num(),
                general::fibonacci(),
                general::wikipedia(),
                general::test_reuse_response()
            ],
            ..Default::default()
        })
        .token(std::env::var("TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();
}