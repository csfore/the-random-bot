//! Description: Commands used to fetch random information using general and to send the
//!              output to Discord.

/*
/// A descriptive description
#[poise::command(slash_command)]
pub async fn command(
    ctx: Context<'_>,
    #[description = "Option 1"] option1: Option<T>,
    #[description = "Option 2"] option2: Option<T>,
) -> Result<(), Error> {
    // Code goes here
    Ok(())
}
*/

use crate::{generators, helpers, Context, Error};
use serde_derive::Deserialize;
use rand::{thread_rng, Rng};

/// Generates a random number between the floor and ceiling you provide
#[poise::command(slash_command, prefix_command)]
pub async fn num(
    ctx: Context<'_>,
    #[description = "Floor"] floor_option: Option<i32>,
    #[description = "Ceiling"] ceiling_option: Option<i32>,
) -> Result<(), Error> {
    let floor = floor_option.as_ref().unwrap_or_else(|| &1);
    let ceiling = ceiling_option.as_ref().unwrap_or_else(|| &100);
    ctx.say(generators::general::rand_num(*floor, *ceiling).to_string())
        .await?;
    Ok(())
}

/// Generates a random number between the floor and ceiling you provide
#[poise::command(slash_command, prefix_command)]
pub async fn fibonacci(
    ctx: Context<'_>,
    #[description = "Nth number in the fibonacci sequence"] digit: Option<i32>,
) -> Result<(), Error> {
    let nth = digit.as_ref().unwrap_or_else(|| &1);
    ctx.say(generators::general::nth_fibo(*nth).to_string()).await?;
    Ok(())
}

/// Generates a random word and definition
#[poise::command(slash_command, prefix_command)]
pub async fn word(ctx: Context<'_>) -> Result<(), Error> {
    let choice = generators::general::word();
    let word = choice.0;
    let definition = choice.1;

    let word_lower = helpers::helpers::capitalize_first_letter(&word.to_lowercase());

    ctx.send(|b| {
        b.embed(|b| {
            b.title(format!("{word_lower}"))
                .description(format!("{definition}"))
                .color(0xB87DDF)
            .footer(|f| f.text("Dictionary contents provided by github.com/adambom/dictionary"))
        })
    })
    .await?;
    Ok(())
}