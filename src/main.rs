mod weather_data;

use clap::{App, Arg};
use reqwest::Error;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let matches = App::new("Weather CLI")
        .about("Gets weather for a specified city")
        .arg(Arg::with_name("CITY")
            .help("The city to get the weather for")
            .required(true)
            .takes_value(true)
            .index(1))
        .get_matches();

    let city = matches.value_of("CITY").unwrap();
    let api_key = env::var("API_KEY").expect("API_KEY not found in .env file");
    let request_url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, api_key);

    match reqwest::get(&request_url).await?.json::<weather_data::Root>().await {
        Ok(data) => {
            data.weather_table();
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    };


    Ok(())
}
