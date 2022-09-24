//! Description: Module to store all commands that use APIs (will probably be own directory later)

extern crate wikipedia;

use crate::{generators, helpers, Context, Error};
use rand::{thread_rng, Rng};
use chrono::{DateTime, TimeZone, Utc};
use serde_derive::{Deserialize, Serialize};

/// A random fact
#[poise::command(slash_command)]
pub async fn fact(ctx: Context<'_>) -> Result<(), Error> {
    #[derive(Debug, Deserialize)]
    struct Response {
        text: String,
    }
    let resp = reqwest::get("https://uselessfacts.jsph.pl/random.json?language=en").await?;
    let body = resp.text().await?;

    let response: Response = serde_json::from_str(&body).unwrap();
    let message = response.text;
    ctx.send(|b| b.embed(|b| b.title("Your Fact:").description(message).color(0xB87DDF).footer(|f| f.text("Facts provided by uselessfacts.jsph.pl"))))
        .await?;
    //println!("{}\n{}", &message, body);
    Ok(())
}

/// Generates a random wikipedia article
#[poise::command(slash_command, prefix_command)]
pub async fn wikipedia(ctx: Context<'_>) -> Result<(), Error> {
    let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
    let page = wiki.random().unwrap().expect("Something");
    let title = &page;

    let result = wiki.page_from_title(String::from(title));
    let content = result.get_summary().unwrap();

    let convert = String::from(title).replace(" ", "_");

    ctx.send(|b| {
        b.embed(|b| b.title(format!("{title}")).description(format!("{content}"))
            .field("URL", format!("https://en.wikipedia.org/wiki/{convert}\nPlease note: We just replace the spaces with underscores so link may be broken"), false)
            .color(0xB87DDF)
            .footer(|f| f.text("Info provided by wikipedia.org"))
        )
    }).await?;
    Ok(())
}

/// A random youtube video
#[poise::command(slash_command)]
pub async fn youtube(ctx: Context<'_>) -> Result<(), Error> {
    let video: String = generators::youtube::youtube_video();
    println!("{}", video);
    ctx.say(video).await?;
    Ok(())
}

/// Generates a reddit post
#[poise::command(slash_command, prefix_command)]
pub async fn reddit(ctx: Context<'_>) -> Result<(), Error> {
    let post = generators::reddit::get_post().await;
    let title = post.title;
    let sub = post.subreddit;
    let text = post.selftext;
    let author = post.author;
    let perma = post.permalink;
    let mut thumb = post.thumbnail;
    let time = Utc.timestamp(post.created.round() as i64, 0);
    // spoiler
    // self
    // default
    let defaults = vec!["spoiler", "self", "default"];
    if defaults.contains(&thumb.as_str()) {
        thumb = String::from("https://i.imgur.com/ws2kAA0.png");
    }

    ctx.send(|b| {
        b.embed(|b| {
            b.title(format!("{title} - /u/{author}"))
                .description(format!("{text}\n\n[Permalink](https://reddit.com{perma})"))
                .thumbnail(format!("{thumb}"))
                .footer(|f| f.text(format!("Created at {time} in /r/{sub}")))
                .color(0xB87DDF)
        })
    })
        .await?;
    Ok(())
}


/// Random weather of a location
#[poise::command(slash_command)]
pub async fn weather(
    ctx: Context<'_>,
    #[description = "Latitude (optional)"] mut lat: Option<f64>,
    #[description = "Longitude (optional)"] mut long: Option<f64>,
) -> Result<(), Error> {
    match lat {
        None => {
            lat = Some(rand::thread_rng().gen_range(-90.0..=90.0));
        },
        _ => ()
    }

    match long {
        None => {
            long = Some(rand::thread_rng().gen_range(-180.0..=180.0));
        },
        _ => ()
    }

    let result = generators::weather::get_weather(lat.unwrap(), long.unwrap()).await;
    let city = generators::weather::get_city(result.coord.lat, result.coord.lon).await.unwrap();
    let mut city_msg = String::new();
    match city {
        None => {
            city_msg = "No City Found".to_string();
        }
        _ => {
            let city_result = &city.unwrap()[0];
            let city_name = city_result.name.as_ref().unwrap().as_str();
            let city_state = city_result.state.as_ref().unwrap().as_str();
            let city_country = city_result.country.as_ref().unwrap().as_str();
            city_msg = format!("{}, {}, {}", city_name, city_state, city_country);
        }
    }
    let icon = &result.weather[0].icon;

    ctx.send(|b| {
        b.embed(|b| {
            b.title(format!("Lat: {}, Long: {} - {}", result.coord.lat, result.coord.lon, city_msg))
                .description(format!("Description: {}\nTemp: {:.2}°F/{:.2}°C/{:.2}K\nFeels Like: {:.2}°C\nPressure: {}hPa", result.weather[0].description, (result.main.temp - 273.15) * 1.8 + 32 as f64, result.main.temp - 273.15, result.main.temp, result.main.feels_like - 273.15, result.main.pressure))
                .thumbnail(format!("https://openweathermap.org/img/wn/{icon}@4x.png"))
                .footer(|f| f.text("Weather data provided by OpenWeather - openweathermap.org"))
                .color(0xB87DDF)
        })
    })
        .await?;
    Ok(())
}