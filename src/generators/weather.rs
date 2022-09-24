//! Description: OpenWeather functions

use crate::generators::structs::{City, Weather};
use crate::helpers::helpers;

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