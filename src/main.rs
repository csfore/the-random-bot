mod generators {
    pub mod num;
    pub mod fibo;
}

use poise::serenity_prelude as serenity;
use dotenv::dotenv;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
struct Data {}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Generates a random number between the floor and ceiling you provide
#[poise::command(slash_command, prefix_command)]
async fn num(
    ctx: Context<'_>,
    #[description = "Floor"] floor: Option<i32>,
    #[description = "Ceiling"] ceiling: Option<i32>,
) -> Result<(), Error> {
    let f = floor.as_ref().unwrap_or_else(|| &1);
    let c = ceiling.as_ref().unwrap_or_else(|| &100);;
    ctx.say(generators::num::rand_num(*f, *c).to_string()).await?;
    Ok(())
}

/// Generates a random number between the floor and ceiling you provide
#[poise::command(slash_command, prefix_command)]
async fn fibonacci(
    ctx: Context<'_>,
    #[description = "Nth number in the fibonacci sequence"] digit: Option<i32>
) -> Result<(), Error> {
    let nth = digit.as_ref().unwrap_or_else(|| &1);
    ctx.say(generators::fibo::nth_fibo(*nth).to_string()).await?;
    Ok(())
}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            // if you get a red line here on the last parenthesis, ignore it
            commands: vec![age(), register(), num(), fibonacci()],
            ..Default::default()
        })
        .token(std::env::var("TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();
}