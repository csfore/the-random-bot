use poise::serenity_prelude as serenity;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


/// Description of the command here
///
/// Here you can explain how the command \
/// is used and how it works.
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
            .title("H")
            .description("Fix me")
        )
        // .ephemeral(true) // this one only applies in application commands though
    ).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                test()
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