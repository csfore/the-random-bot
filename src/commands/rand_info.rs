extern crate wikipedia;

use crate::{Context, Error, generators};
use poise::serenity_prelude as serenity;

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

/// Generates a random wikipedia article
#[poise::command(slash_command, prefix_command)]
pub async fn wikipedia(
    ctx: Context<'_>
) -> Result<(), Error> {
    let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
    let page = wiki.random().unwrap().expect("Something");
    let title = &page;

    let result = wiki.page_from_title(String::from(title));
    let content = result.get_summary().unwrap();

    let convert = String::from(title).replace(" ", "_");

    ctx.send(|b| {
        b.embed(|b| b.title(format!("{title}")).description(format!("{content}"))
            .field("URL", format!("https://en.wikipedia.org/wiki/{convert}\nPlease note: We just replace the spaces with underscores so link may be broken"), false)
            .color(0xB87DDF))
    }).await?;
    Ok(())

}