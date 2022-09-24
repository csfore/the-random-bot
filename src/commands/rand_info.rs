//! Description: Commands used to fetch random information using generators.rs and to send the
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

extern crate wikipedia;

use chrono::{DateTime, TimeZone, Utc};
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
    ctx.say(generators::rand_num(*floor, *ceiling).to_string())
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
    ctx.say(generators::nth_fibo(*nth).to_string()).await?;
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

/// Generates a random word and definition
#[poise::command(slash_command, prefix_command)]
pub async fn word(ctx: Context<'_>) -> Result<(), Error> {
    let choice = generators::word();
    let word = choice.0;
    let definition = choice.1;

    let word_lower = helpers::capitalize_first_letter(&word.to_lowercase());

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

/// A random youtube video
#[poise::command(slash_command)]
pub async fn youtube(ctx: Context<'_>) -> Result<(), Error> {
    let video: String = generators::youtube_video();
    println!("{}", video);
    ctx.say(video).await?;
    Ok(())
}

/// Generates a reddit post
#[poise::command(slash_command, prefix_command)]
pub async fn reddit(ctx: Context<'_>) -> Result<(), Error> {
    let post = generators::get_post().await;
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
        Option::None => {
            lat = Some(rand::thread_rng().gen_range(-90.0..=90.0));
        },
        _ => ()
    }

    match long {
        Option::None => {
            long = Some(rand::thread_rng().gen_range(-180.0..=180.0));
        },
        _ => ()
    }

    let result = generators::get_weather(lat.unwrap(), long.unwrap()).await;
    let city = generators::get_city(result.coord.lat, result.coord.lon).await.unwrap();
    let mut city_msg = String::new();
    match city {
        Option::None => {
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