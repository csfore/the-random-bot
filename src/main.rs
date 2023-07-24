use poise::serenity_prelude as serenity;
use std::collections::HashMap;
use rand::{
    Rng,
    seq::SliceRandom
};
use serde::{Deserialize, Serialize};

const BRAND_COLOR: i32 = 0xB87DDF;

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

/// Random Word generator
#[poise::command(prefix_command, slash_command)]
async fn word(ctx: Context<'_>) -> Result<(), Error> {
    let config_path = "dictionary.json";
    let config_read = std::fs::read_to_string(&config_path);

    let dictionary: HashMap<String, String> = serde_json::from_str(&config_read.expect("dictionary.json not found in the top level directory.")).expect("dictionary.json wasn't able to be parsed");

    let mut keys: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();

    for (key, value) in dictionary {
        keys.push(key);
        values.push(value);
    }

    let choice = rand::thread_rng().gen_range(0..keys.len() - 1);

    let keys_ret = &keys[choice];
    let values_ret = &values[choice];

    ctx.send(|f| f
        .embed(|f| f
            .title("Your word is....")
            .description(format!("{} - {}", keys_ret, values_ret))
            .color(BRAND_COLOR))
    ).await?;
    Ok(())

}

/// Random Number Generator
#[poise::command(prefix_command, slash_command)]
async fn number(
    ctx: Context<'_>,
    #[description = "The number to start at"] floor: Option<i128>,
    #[description = "The number to end at"] ceiling: Option<i128>
) -> Result<(), Error> {
    let mut floor_raw: i128 = 0;
    let mut ceiling_raw = 10;

    if let (Some(floor), Some(ceiling)) = (floor, ceiling) {
        floor_raw = floor;
        ceiling_raw = ceiling;
    }

    let num: i128 = rand::thread_rng().gen_range(floor_raw..=ceiling_raw);

    ctx.send(|f| f
        .embed(|f| f
            .title(format!("Number between {} and {}", floor_raw, ceiling_raw))
            .description(format!("{}", num))
            .color(BRAND_COLOR))
    ).await?;
    Ok(())
}

/// Ask me anything, and I may respond wisely
#[poise::command(prefix_command, slash_command)]
async fn ask(
    ctx: Context<'_>,
    question: String
) -> Result<(), Error> {
    let mut answer = "";

    let questions: Vec<&str> = vec![
        "It is certain.", "Reply hazy, try again", 
        "Don't count on it", "It is decidedly so", "Ask again later", 
        "My reply is no", "Without a doubt", "Better not tell you now", 
        "My sources say no", "Yes, definitely", "Cannot predict now", 
        "Outlook not so good", "You may rely on it", "Concentrate and ask again",
        "Very doubtful", "As I see it, yes", "Most likely", "Outlook good", "Yes",
        "Signs point to yes"
    ];

    if let Some(answer_raw) = questions.choose(&mut rand::thread_rng()) {
        answer = *answer_raw;
    }

    ctx.send(|f| f
        .embed(|f| f
            .title("The 8 Ball responds...")
            .description(format!("{}\n\nYou asked: **{}**", answer, question))
            .color(BRAND_COLOR))).await?;

    Ok(())
}

#[tokio::main]
async fn main() {   

    let config_file = std::fs::read_to_string("config.json").expect("config.json file not found in top directory"); 
    let config_dict: HashMap<String, String> = serde_json::from_str(&config_file).expect("config.json couldn't be parsed. Check the file format"); 

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                test(),
                number(),
                ask(),
                word()
            ],
            ..Default::default()
        })
        .token(config_dict.get("DISCORD_TOKEN").expect("missing DISCORD_TOKEN in config.json"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}