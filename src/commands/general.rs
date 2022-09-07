use crate::{Context, Error};
use poise::serenity_prelude as serenity;

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