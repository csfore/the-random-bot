//! File to store all the structs

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawPost {
    pub(crate) data: Listing
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Listing {
    pub(crate) children: Vec<Children>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Children {
    pub(crate) data: PostData
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

/*
    OpenWeather Structs
 */

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