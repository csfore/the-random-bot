use poise::serenity_prelude as serenity;
use rand::Rng;
use serde::{Deserialize, Serialize};

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Serialize, Deserialize)]
struct Config {
    token: String
}

/// Test functionality
#[poise::command(prefix_command, slash_command /* add more optional command settings here, like slash_command */)]
async fn test(
    ctx: Context<'_>,
    // #[description = "Description of arg1 here"] arg1: serenity::Member,
    // #[description = "Description of arg2 here"] arg2: Option<u32>,
) -> Result<(), Error> {
    // Command code here
    ctx.send(|f| f
        .content("Text")
        .embed(|f| f
            .title("Hello World!")
            .description("Fix me")
        )
        // .ephemeral(true) // this one only applies in application commands though
    ).await?;
    Ok(())
}

/// Random Number Generator
#[poise::command(prefix_command, slash_command)]
async fn number(
    ctx: Context<'_>,
    #[description = "The number to start at"] floor: Option<u128>,
    #[description = "The number to end at"] ceiling: Option<u128>
) -> Result<(), Error> {
    let mut floor_raw = 0;
    let mut ceiling_raw = 10;

    if let (Some(floor), Some(ceiling)) = (floor, ceiling) {
        floor_raw = floor;
        ceiling_raw = ceiling;
    }

    let num: u128 = rand::thread_rng().gen_range(floor_raw..=ceiling_raw);

    ctx.send(|f| f
        .embed(|f| f
            .title(format!("Number between {} and {}", floor_raw, ceiling_raw))
            .description(format!("{}", num))
            .color(0xB87DDF))
    ).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                test(),
                number()
            ],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}