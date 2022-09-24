//! Description: Generators used to supply commands with information
use roux::Subreddit;
use rand::{seq::SliceRandom, Rng, thread_rng};
use youtube_dl::SearchOptions;
use youtube_dl::{YoutubeDl, YoutubeDlOutput};
use serde_derive::{Deserialize, Serialize};
use crate::helpers;

use std::collections::HashMap;

extern crate wikipedia;

#[derive(Serialize, Deserialize, Debug)]
struct RawPost {
    data: Listing
}

#[derive(Serialize, Deserialize, Debug)]
struct Listing {
    children: Vec<Children>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Children {
    data: PostData
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clouds {
    pub all: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    pub speed: f64,
    pub deg: i64,
    pub gust: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i64,
    pub humidity: i64,
    pub sea_level: i64,
    pub grnd_level: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherDesc {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherCoords {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub coord: WeatherCoords,
    pub weather: Vec<WeatherDesc>,
    pub base: String,
    pub main: Main,
    pub visibility: i64,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i64,
    pub id: i64,
    pub name: String,
    pub cod: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalNames {
    pub ar: Option<String>,
    pub ascii: Option<String>,
    pub bg: Option<String>,
    pub ca: Option<String>,
    pub de: Option<String>,
    pub el: Option<String>,
    pub en: Option<String>,
    pub fa: Option<String>,
    pub feature_name: Option<String>,
    pub fi: Option<String>,
    pub fr: Option<String>,
    pub gl: Option<String>,
    pub he: Option<String>,
    pub hi: Option<String>,
    pub id: Option<String>,
    pub it: Option<String>,
    pub ja: Option<String>,
    pub la: Option<String>,
    pub lt: Option<String>,
    pub pt: Option<String>,
    pub ru: Option<String>,
    pub sr: Option<String>,
    pub th: Option<String>,
    pub tr: Option<String>,
    pub vi: Option<String>,
    pub zu: Option<String>,
    pub zh: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct City {
    pub name: Option<String>,
    pub local_names: Option<LocalNames>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub country: Option<String>,
    pub state: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PostData {
    pub subreddit: String,
    pub selftext: String,
    pub author: String,
    pub permalink: String,
    pub url: Option<String>,
    pub title: String,
    pub created: f64,
    pub thumbnail: String,
}

pub fn nth_fibo(mut nth: i32) -> i32 {
    if nth == 0 {
        return 1;
    } else if nth == 1 {
        return 2;
    } else {
        let mut num1 = 0;
        let mut num2 = 1;

        while nth > 1 {
            let next: u32 = num1 + num2;

            num1 = num2;
            num2 = next;

            nth -= 1;
        }
        return num2.try_into().unwrap();
    }
}

pub fn rand_num(floor: i32, ceiling: i32) -> i32 {
    return rand::thread_rng().gen_range(floor..=ceiling);
}

pub fn eight_ball() -> &'static str {
    let responses = [
        "It is certain",
        "Without a doubt",
        "You may rely on it",
        "Yes, definitely",
        "It is decidedly so",
        "As I see it, yes",
        "Most likely",
        "Yes",
        "Outlook good",
        "Signs point to yes",
        "Reply hazy try again",
        "Better not tell you now",
        "Ask again later",
        "Cannot predict now",
        "Concentrate and ask again",
        "Don't count on it",
        "Outlook not so good",
        "My sources say no",
        "Very doubtful",
        "My reply is no",
    ];
    let index = rand::thread_rng().gen_range(0..=responses.len() - 1);
    return responses[index];
}

pub fn word() -> (String, String) {
    let config_path = "dictionary.json";
    let config_read = std::fs::read_to_string(&config_path);

    let dictionary: HashMap<String, String> = serde_json::from_str(&config_read.unwrap()).unwrap();

    let mut keys: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();

    for (key, value) in dictionary {
        keys.push(key);
        values.push(value);
    }

    let choice = rand::thread_rng().gen_range(0..keys.len() - 1);

    let keys_ret = &keys[choice];
    let values_ret = &values[choice];

    return (String::from(keys_ret), String::from(values_ret));
}

pub fn youtube_video() -> String {
    let mut links: Vec<String> = vec![];

    let charset = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
        "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "1", "2",
        "3", "4", "5", "6", "7", "8", "9", "0",
    ];

    let rand_prefix = charset.choose(&mut rand::thread_rng()).unwrap();
    //let rand_postfix = charset.choose(&mut rand::thread_rng()).unwrap();

    //let random_int: i32 = rand::thread_rng().gen_range(0..=9999);

    let search_string: String = rand_prefix.to_string();

    let search = SearchOptions::youtube(search_string).with_count(1);
    let output = YoutubeDl::search_for(&search)
        .socket_timeout("15")
        .run()
        .unwrap();

    let entries = match output {
        YoutubeDlOutput::Playlist(videos) => videos.entries,
        _ => panic!("single video should not be a playlist"),
    };

    for video in entries.unwrap() {
        links.push(video.webpage_url.unwrap());
    }

    return String::from(links.choose(&mut rand::thread_rng()).unwrap());
}

pub async fn get_post() -> PostData {
    let subreddit = Subreddit::new("all");

    // Get new posts with limit = 5
    let get_new = subreddit.latest(5, None).await;

    let jobj = serde_json::to_string(&get_new.unwrap());
    let post: RawPost = serde_json::from_str(&jobj.unwrap()).unwrap();
    //    for posts in post.data.children {
    //        println!("{:#?}", posts.data);
    //    }
    let rand_post = post.data.children.choose(&mut rand::thread_rng()).unwrap();

    let newobj = serde_json::to_string(&rand_post.data);
    let owned_post: PostData = serde_json::from_str(&newobj.unwrap()).unwrap();
    return owned_post
}

pub async fn get_weather(lat: f64, long: f64) -> Weather {
    let document = request(lat, long).await.unwrap();
    let weather: Weather = serde_json::from_str(&document).unwrap();
    return weather
}

pub async fn get_city(lat: f64, lon: f64) -> Result<Option<Vec<City>>, ()> {
    let reverse = city_reversed(lat, lon).await.unwrap();
    let city: Vec<City> = serde_json::from_str(&reverse).unwrap();
//    if city[0].name.as_ref().unwrap() == "" {
//        println!("No results");
//        return None;
//    }
    if city.len() == 0 {
        return Ok(None);
    }
    match city[0].name {
        Option::None => {
            return Ok(None)
        }
        _ => {
            return Ok(Some(city))
        }
    }
}

async fn request(lat: f64, long: f64) -> Result<String, std::io::Error> {
    let key = helpers::get_ow().await.unwrap();
    let body = reqwest::get(format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}", lat, long, key))
    .await
    .unwrap()
    .text()
    .await;
    Ok(body.unwrap())
}

async fn city_reversed(lat: f64, long: f64) -> Result<String, std::io::Error> {
    let key = helpers::get_ow().await.unwrap();
    let body = reqwest::get(format!("http://api.openweathermap.org/geo/1.0/reverse?lat={}&lon={}&limit=5&appid={}", lat, long, key))
    .await
    .unwrap()
    .text()
    .await;
    Ok(body.unwrap())
}

// TODO: Figure out a way to fix this in regards to floating point stuff
// pub fn nth_pi(nth: i32) -> f64 {
//     /// Returns the Nth digit of Pi giving the nth argument
//     ///
//
//     /// Do not use this for now until we find a way to use variable floating point precision
//     ///
//     /// Python has a library called `decimal` that let's you calculate to the Nth decimal, while Rust is
//     /// confined to f32 and f64. For now I'll keep this here in case we do find something in the future.
//     let mut c: f64 = 426880_f64 * 10005_f64.sqrt();
//     let mut l: f64 = 13591409 as f64;
//     let mut x: f64 = 1 as f64;
//     let mut m: f64 = 1 as f64;
//     let mut k: f64 = 6 as f64;
//     let mut s: f64 = l as f64;
//     for i in 1..nth {
//         m = m * (k.powi(3) - 16_f64 * k) / f64::from(((i + 1).pow(3)));
//         l += 545140134.0;
//         x *= -262537412640768000.0;
//         s += (m * l) / x;
//     }
//     let pi = c / s;
//     println!("{}", pi);
//     return pi;
// }
