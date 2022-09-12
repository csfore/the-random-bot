//! Description: Commands used to fetch random information using generators.rs and to send the
//!              output to Discord.


extern crate wikipedia;

use crate::{Context, Error, generators, helpers};
use serde_derive::{Deserialize};

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

/// Generates a random word and definition
#[poise::command(slash_command, prefix_command)]
pub async fn word(
    ctx: Context<'_>
) -> Result<(), Error> {
    let choice = generators::word();
    let word = choice.0;
    let definition = choice.1;

    let word_lower = helpers::capitalize_first_letter(&word.to_lowercase());

    ctx.send(|b| {
        b.embed(|b| b.title(format!("{word_lower}"))
            .description(format!("{definition}"))
            .color(0xB87DDF))
    }).await?;
    Ok(())
}

/// A random fact
#[poise::command(slash_command)]
pub async fn fact(
    ctx: Context<'_>
) -> Result<(), Error> {
    #[derive(Debug, Deserialize)]
    struct Response {
        text: String
    }
    let resp = reqwest::get("https://uselessfacts.jsph.pl/random.json?language=en").await?;
    let body = resp.text().await?;

    let response: Response = serde_json::from_str(&body).unwrap();
    let message = response.text;
    ctx.send(|b| {
        b.embed(|b| b.title("Your Fact:")
            .description(message)
            .color(0xB87DDF))
    }).await?;
    //println!("{}\n{}", &message, body);
    Ok(())
}