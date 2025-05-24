use clap::Parser;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Parser)]
struct Cli{
    city: String,
}

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
}

#[derive(Deserialize, Debug)]
struct Weather {
    main: String,
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: u64,
}

fn get_weather(city: &str, api_key: &str) -> Result<WeatherResponse, Box<dyn Error>> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=imperial",
        city, api_key
    );

    let client = Client::new();
    let resp = client.get(url).send()?.text()?;
    // println!("Raw response: {}", resp);

    // Deserialize the JSON response into the WeatherResponse struct
    let weather_resp: WeatherResponse = serde_json::from_str(&resp)?;
    Ok(weather_resp)
}

fn main() {
    let args = Cli::parse();
    let api_key = "22e2c75420ca34b5e47e6057a5715164";

    match get_weather(&args.city, api_key) {
        Ok(weather) => {
            println!("Weather in {}:", args.city);
            println!(
                "{} - {}, Temp: {}°F, Humidity: {}%",
                weather.weather[0].main,
                weather.weather[0].description,
                weather.main.temp,
                weather.main.humidity
            );
        }
        Err(e) => eprintln!("Error fetching weather data: {}", e),
    }
}