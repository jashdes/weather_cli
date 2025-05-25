use clap::Parser;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;
use std::env;
use dotenv::dotenv;
use colored::*;

#[derive(Parser)]
struct Cli{
    city: String,

    #[arg(short, long, action)]
    forecast: bool,
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
    feels_like: f64,
    humidity: u64,
}

#[derive(Deserialize, Debug)]
struct ForecastResponse {
    list: Vec<ForecastItem>,
}

#[derive(Deserialize, Debug)]
struct ForecastItem {
    dt_txt: String,
    main: Main,
    weather: Vec<Weather>,
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

fn get_forecast(city: &str, api_key: &str) -> Result<ForecastResponse, Box<dyn Error>> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/forecast?q={}&appid={}&units=imperial",
        city, api_key
    );

    let client = Client::new();
    let resp = client.get(url).send()?.text()?;
    // println!("Raw response: {}", resp);

    // Deserialize the JSON response into the ForecastResponse struct
    let forecast_resp: ForecastResponse = serde_json::from_str(&resp)?;
    Ok(forecast_resp)
}

fn main() {
    dotenv().ok();
    let args = Cli::parse();
    let api_key = env::var("OPENWEATHER_API_KEY").expect("API key not set in .env file");

    if args.forecast {
        match get_forecast(&args.city, &api_key) {
            Ok(forecast) => {
                println!("{}", "🌤 5-Day Forecast:".bright_magenta().bold());
                for item in forecast.list.iter().take(5) {
                    println!(
                        "{} | {} - {}, Temp: {}°F",
                        item.dt_txt.yellow(),
                        item.weather[0].main.green(),
                        item.weather[0].description.cyan(),
                        item.main.temp.to_string().red().bold()
                    );
                }
            }
            Err(e) => eprintln!("🥲 Oops! Couldn't fetch forecast data. {}", e),
        }
    } else {
        match get_weather(&args.city, &api_key) {
            Ok(weather) => {
                println!("{}", "Weather CLI 🍷✨".bright_magenta().bold());
                println!("Weather in {}:", args.city.green().bold());
                println!(
                    "{} - {}, Temp: {}°F (Feels like: {}°F), Humidity: {}%",
                    weather.weather[0].main.yellow(),
                    weather.weather[0].description.cyan(),
                    weather.main.temp.to_string().red().bold(),
                    weather.main.feels_like.to_string().magenta().bold(),
                    weather.main.humidity.to_string().blue()
                );
            }
            Err(e) => eprintln!("🥲 Oops! Couldn't fetch weather data. {}", e),
        }
    }
}