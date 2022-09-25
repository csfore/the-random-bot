//! General purpose commands

use crate::{generators, Context, Error};
use poise::serenity_prelude as serenity;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    banned_words: Vec<String>,
}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn say(
    ctx: Context<'_>,
    #[description = "Tell the bot to say something"] message: Option<String>,
) -> Result<(), Error> {
    let config_path = "config.json";
    let config_read = std::fs::read_to_string(&config_path);

    let config: Config = serde_json::from_str(&config_read.unwrap()).unwrap();

    let say = message.unwrap();
    if config.banned_words.contains(&String::from(&say)) {
        ctx.say("Banned word").await?;
        return Ok(());
    } else {
        ctx.say(say).await?;
    }
    Ok(())
}

#[poise::command(slash_command)]
pub async fn ask(
    ctx: Context<'_>,
    #[description = "Ask me anything, and I may respond wisely"] question: Option<String>,
) -> Result<(), Error> {
    let ask = question.unwrap();
    let asker = String::from(&ctx.author().name) + "#" + &ctx.author().discriminator.to_string();

    let answer = generators::general::eight_ball();
    ctx.send(|b| {
        b.embed(|b| {
            b.title(format!("{asker} Asked: {ask}"))
                .description(format!("{answer}"))
                .color(0xB87DDF)
        })
    })
    .await?;
    Ok(())
}

/*
    Function below is used for testing purposes, uncomment it as needed and comment it back
    on push otherwise it'll probably create more issues than not if you leave it uncommented.
*/

// #[poise::command(slash_command, prefix_command, reuse_response)]
// pub async fn test(ctx: Context<'_>) -> Result<(), Error> {
//     use std::{thread, time};
//
//     let ten_millis = time::Duration::from_millis(10000);
//     let now = time::Instant::now();
//
//     thread::sleep(ten_millis);
//     println!("Slept");
//     Ok(())
// }
