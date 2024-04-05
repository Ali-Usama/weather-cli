/// The returned data from the OpenWeatherMap API looks like this:
/*
{
    "coord": {
        "lon": 73.1338,
        "lat": 33.7104
    },
    "weather": [
        {
         "id": 800,
         "main": "Clear",
         "description": "clear sky",
        "icon": "01d"
    }
    ],
    "base": "stations",
    "main": {
        "temp": 28.24,
        "feels_like": 27.61,
        "temp_min": 28.24,
        "temp_max": 28.24,
        "pressure": 1009,
        "humidity": 36,
        "sea_level": 1009,
        "grnd_level": 929
    },
    "visibility": 10000,
    "wind": {
        "speed": 4.35,
        "deg": 297,
        "gust": 2.9
    },
    "clouds": {
        "all": 0
    },
    "dt": 1712306144,
    "sys": {
        "type": 2,
        "id": 2007435,
        "country": "PK",
        "sunrise": 1712278162,
        "sunset": 1712323829
    },
    "timezone": 18000,
    "id": 1162015,
    "name": "Islamabad",
    "cod": 200
}
 */


use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    #[serde(rename = "type")]
    pub r#type: i64,
    pub id: i64,
    pub country: String,
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
pub struct WeatherStruct {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    pub coord: Coord,
    pub weather: Vec<WeatherStruct>,
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

impl Root {
    pub fn weather_table(&self) {
        let mut weather_table = prettytable::Table::new();
        weather_table.add_row(prettytable::row!["City", "Temperature", "Feels Like", "Wind Speed", "Minimum Temperature", "Maximum Temperature", "Humidity"]);
        weather_table.add_row(prettytable::row![&self.name, &self.main.temp, &self.main.feels_like, &self.wind.speed, &self.main.temp_min, &self.main.temp_max, &self.main.humidity]);
        weather_table.printstd();
    }
}