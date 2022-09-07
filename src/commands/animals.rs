use crate::{Context, Error};
use serde_derive::{Deserialize};

/// Fetches you a dog
#[poise::command(slash_command, prefix_command)]
pub async fn dog(
    ctx: Context<'_>
) -> Result<(), Error> {
    #[derive(Debug, Deserialize)]
    struct Response {
        message: String
    }

    let resp = reqwest::get("https://dog.ceo/api/breeds/image/random").await?;

    let body = resp.text().await?;

    let response: Response = serde_json::from_str(&body).unwrap();
    let message = response.message;
    ctx.send(|b| {
        b.embed(|b| b.title("Dog!")
            .image(message)
            .color(0xB87DDF))
    }).await?;
    Ok(())
}

/// Fetches you a fox
#[poise::command(slash_command)]
pub async fn fox(
    ctx: Context<'_>
) -> Result<(), Error> {
    #[derive(Debug, Deserialize)]
    struct Response {
        image: String
    }

    let resp = reqwest::get("https://randomfox.ca/floof/").await?;

    let body = resp.text().await?;

    let response: Response = serde_json::from_str(&body).unwrap();
    let message = response.image;
    ctx.send(|b| {
        b.embed(|b| b.title("Fox!")
            .image(message)
            .color(0xB87DDF))
    }).await?;
    Ok(())
}

/// Fetches you a cat
#[poise::command(slash_command)]
pub async fn cat(
    ctx: Context<'_>
) -> Result<(), Error> {
    #[derive(Debug, Deserialize)]
    struct Response {
        link: String
    }

    let resp = reqwest::get("https://some-random-api.ml/img/cat").await?;

    let body = resp.text().await?;

    let response: Response = serde_json::from_str(&body).unwrap();
    let message = response.link;
    ctx.send(|b| {
        b.embed(|b| b.title("Cat!")
            .image(message)
            .color(0xB87DDF))
    }).await?;
    Ok(())
}

/// Fetches you a panda
#[poise::command(slash_command)]
pub async fn panda(
    ctx: Context<'_>
) -> Result<(), Error> {
    #[derive(Debug, Deserialize)]
    struct Response {
        link: String
    }

    let resp = reqwest::get("https://some-random-api.ml/img/panda").await?;

    let body = resp.text().await?;

    let response: Response = serde_json::from_str(&body).unwrap();
    let message = response.link;
    ctx.send(|b| {
        b.embed(|b| b.title("Panda!")
            .image(message)
            .color(0xB87DDF))
    }).await?;
    Ok(())
}

/// Fetches you a red panda
#[poise::command(slash_command)]
pub async fn red_panda(
    ctx: Context<'_>
) -> Result<(), Error> {
    #[derive(Debug, Deserialize)]
    struct Response {
        link: String
    }

    let resp = reqwest::get("https://some-random-api.ml/img/red_panda").await?;

    let body = resp.text().await?;

    let response: Response = serde_json::from_str(&body).unwrap();
    let message = response.link;
    ctx.send(|b| {
        b.embed(|b| b.title("Red Panda!")
            .image(message)
            .color(0xB87DDF))
    }).await?;
    Ok(())
}

/// Fetches you a birb
#[poise::command(slash_command)]
pub async fn bird(
    ctx: Context<'_>
) -> Result<(), Error> {
    #[derive(Debug, Deserialize)]
    struct Response {
        link: String
    }

    let resp = reqwest::get("https://some-random-api.ml/img/birb").await?;

    let body = resp.text().await?;

    let response: Response = serde_json::from_str(&body).unwrap();
    let message = response.link;
    ctx.send(|b| {
        b.embed(|b| b.title("Birb!")
            .image(message)
            .color(0xB87DDF))
    }).await?;
    Ok(())
}

/// Fetches you a Koala
#[poise::command(slash_command)]
pub async fn koala(
    ctx: Context<'_>
) -> Result<(), Error> {
    #[derive(Debug, Deserialize)]
    struct Response {
        link: String
    }

    let resp = reqwest::get("https://some-random-api.ml/img/koala").await?;

    let body = resp.text().await?;

    let response: Response = serde_json::from_str(&body).unwrap();
    let message = response.link;
    ctx.send(|b| {
        b.embed(|b| b.title("Koala!")
            .image(message)
            .color(0xB87DDF))
    }).await?;
    Ok(())
}