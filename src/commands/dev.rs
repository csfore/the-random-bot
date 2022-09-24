// use crate::{Context, Error, generators};
// use poise::serenity_prelude as serenity;
// use std::fmt::Write as _;
//
// /// Displays your or another user's account creation date
// #[poise::command(slash_command, prefix_command)]
// pub async fn age(
//     ctx: Context<'_>,
//     #[description = "Selected user"] user: Option<serenity::User>,
// ) -> Result<(), Error> {
//     let u = user.as_ref().unwrap_or_else(|| ctx.author());
//     let response = format!("{}'s account was created at {}", u.name, u.created_at());
//     ctx.say(response).await?;
//     Ok(())
// }

use crate::helpers::check_dev;
use crate::{Context, Error};

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    /*
        Built-in poise command to register all slash commands globally or only in-guild
    */
    let author = u64::from(ctx.author().id).to_string();
    if check_dev(author.as_str()).await.unwrap() {
        poise::builtins::register_application_commands_buttons(ctx).await?;
        warn!("Commands registered by {}", ctx.author().name);
        Ok(())
    } else {
        let name = &(ctx.author().name.to_owned() + &ctx.author().discriminator.to_string());
        warn!("User {} tried to register commands", name);
        ctx.say("It seems you don't have permission to use this.")
            .await?;
        Ok(())
    }
}

#[poise::command(prefix_command)]
pub async fn servers(ctx: Context<'_>) -> Result<(), Error> {
    /*
    Built-in poise command to register all slash commands globally or only in-guild
    */
    let author = u64::from(ctx.author().id).to_string();
    if check_dev(author.as_str()).await.unwrap() {
        poise::builtins::servers(ctx).await?;
    } else {
        let name = &(ctx.author().name.to_owned() + &ctx.author().discriminator.to_string());
        warn!("User {} tried to register commands", name);
        ctx.say("It seems you don't have permission to use this.")
            .await?;
    }
    Ok(())
}
