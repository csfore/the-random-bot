use crate::{Context, Error, generators};
use poise::serenity_prelude as serenity;
use serde_derive::{Deserialize};

#[derive(Deserialize, Debug)]
struct Config {
    // last_fm_key: String,
    // last_fm_ua: String,
    //discord_token: String,
    banned_words: Vec<String>,
    //developers: Vec<String>,
    //reddit: Reddit,
    //imgur: Imgur
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
    #[description = "Ask me anything, and I may respond wisely"] question: Option<String>
) -> Result<(), Error> {
    let ask = question.unwrap();
    let asker =String::from(&ctx.author().name) + "#" + &ctx.author().discriminator.to_string();

    let answer = generators::eight_ball();
    ctx.send(|b| {
        b.embed(|b| b.title(format!("{asker} Asked: {ask}")).description(format!("{answer}"))
            .color(0xB87DDF))
    }).await?;
    Ok(())

}

/*
    Function below is used for testing purposes, uncomment it as needed and comment it back
    on push otherwise it'll probably create more issues than not if you leave it uncommented.
 */

// #[poise::command(slash_command, prefix_command, reuse_response)]
// pub async fn test(ctx: Context<'_>) -> Result<(), Error> {
//     let image_url = "https://raw.githubusercontent.com/serenity-rs/serenity/current/logo.png";
//     let title = "Title";
//     let text = "Hello!";
//
//
//     Ok(())
// }

/*
|b| {
        b.content("message 1")
            .embed(|b| b.description("embed 1").image(image_url))
            .components(|b| {
                b.create_action_row(|b| {
                    b.create_button(|b| {
                        b.label("button 1")
                            .style(serenity::ButtonStyle::Primary)
                            .custom_id(1)
                    })
                })
            })
    })
    .await?;
 */