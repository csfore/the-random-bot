//! Developer commands meant to be used only by developers for diagnostic purposes

use crate::{Context, Error};
use crate::helpers::helpers::check_dev;

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
    Built-in poise command to get the server count the bot is in
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
