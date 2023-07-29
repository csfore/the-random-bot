use poise::serenity_prelude as serenity;
use std::collections::HashMap;
use rand::{
    Rng,
    seq::SliceRandom
};
use serde::{Deserialize, Serialize};
use image::{ImageBuffer, RgbImage, Rgb}; 

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

#[poise::command(prefix_command, slash_command)]
async fn color(ctx: Context<'_>) -> Result<(), Error> {

    let red:u8 = rand::thread_rng().gen_range(0..=255);
    let green:u8 = rand::thread_rng().gen_range(0..=255);
    let blue:u8 = rand::thread_rng().gen_range(0..=255);
    let mut img: RgbImage = ImageBuffer::new(128, 128);
    for x in 0..128 {
        for y in 0..128 {
            img.put_pixel(x, y, Rgb([red, green, blue]));
        }
    }
    img.save("./color.jpg")?;
    let img_reader = tokio::fs::File::open("./color.jpg").await?;
    // this works, trust
    let color = i32::from(red) * 256 * 256 + i32::from(green) * 256 + i32::from(blue); 
    ctx.send(|f| f
        .embed(|f| f
            .title("Your color is...")
            .description(format!("RGB = ({0},{1},{2})\n Hex = 0x{0:X}{1:X}{2:X}", red, green, blue))
            .color(color)
            .attachment("color.jpg"))
        .attachment(serenity::AttachmentType::File { file: (&img_reader), filename: ("color.jpg".to_string()) })
        
    ).await?;
    Ok(())

} 

/// Random Word generator
#[poise::command(prefix_command, slash_command)]
async fn word(ctx: Context<'_>) -> Result<(), Error> {
    let dictionary_path = "dictionary.json";
    let dictionary_str = std::fs::read_to_string(&dictionary_path).expect("dictionary.json not found in the top level directory.");

    let dictionary: HashMap<String, String> = serde_json::from_str(&dictionary_str).expect("dictionary.json wasn't able to be parsed");

    let mut words: Vec<String> = Vec::new();
    let mut definitions: Vec<String> = Vec::new();

    for (key, value) in dictionary {
        words.push(key);
        definitions.push(value);
    }

    let choice = rand::thread_rng().gen_range(0..words.len() - 1);

    let word = &words[choice];
    let definition = &definitions[choice];

    ctx.send(|f| f
        .embed(|f| f
            .title("Your word is....")
            .description(format!("{} - {}", word, definition))
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
                word(),
                color()
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