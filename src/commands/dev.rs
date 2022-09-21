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
use crate::{Context, Error};
use crate::helpers::check_dev;

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    /*
        Built-in poise command to register all slash commands globally or only in-guild
    */
    let author = u64::from(ctx.author().id).to_string();
    if check_dev(author.as_str()).await.unwrap() {
        println!("Commands registered.");
        poise::builtins::register_application_commands_buttons(ctx).await?;
    } else {
        println!("Commands failed to register");
        ctx.say("It seems you don't have permission to use this.").await?;
    }

    Ok(())
}
