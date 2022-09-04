extern crate wikipedia;

use crate::{Context, Error, generators};
use poise::serenity_prelude as serenity;
use std::fmt::Write as _;

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

/// Generates a random number between the floor and ceiling you provide
#[poise::command(slash_command, prefix_command)]
pub async fn num(
    ctx: Context<'_>,
    #[description = "Floor"] floor_option: Option<i32>,
    #[description = "Ceiling"] ceiling_option: Option<i32>,
) -> Result<(), Error> {
    let floor = floor_option.as_ref().unwrap_or_else(|| &1);
    let ceiling = ceiling_option.as_ref().unwrap_or_else(|| &100);
    ctx.say(generators::rand_num(*floor, *ceiling).to_string()).await?;
    Ok(())
}

/// Generates a random number between the floor and ceiling you provide
#[poise::command(slash_command, prefix_command)]
pub async fn fibonacci(
    ctx: Context<'_>,
    #[description = "Nth number in the fibonacci sequence"] digit: Option<i32>
) -> Result<(), Error> {
    let nth = digit.as_ref().unwrap_or_else(|| &1);
    ctx.say(generators::nth_fibo(*nth).to_string()).await?;
    Ok(())
}

// /// Generates a random wikipedia article
// #[poise::command(slash_command, prefix_command)]
// async fn wikipedia(
//     ctx: Context<'_>
// ) -> Result<(), Error> {
//     let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
//     let page = wiki.random();
//     let title = page.unwrap().expect("Something");
//     let result = wiki.page_from_title(t);
//     let content = result.get_content().unwrap();
//     //let content = result.get_content().unwrap();
//     ctx.say(format!("Title: {title}\nSummary: {content}\n")).await?;
//     Ok(())
// }