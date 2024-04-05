# Weather CLI

Weather CLI is a command-line interface application written in Rust, designed to fetch and display the weather information for any specified city. Utilizing the OpenWeatherMap API, it provides current weather details such as temperature, wind speed, and humidity.

## Features

- Get current weather information for a specified city
- Displays temperature, feels like, wind speed, minimum and maximum temperature, and humidity
- Easy to use command-line interface

## Prerequisites

Before you begin, ensure you have met the following requirements:
- Rust and Cargo installed on your machine (see [Rust installation guide](https://www.rust-lang.org/tools/install))
- An API key from OpenWeatherMap (register at [OpenWeatherMap](https://openweathermap.org/appid) to get your API key)

## Installation

1. Clone the repository:
```bash
git clone https://yourrepository/weather-cli.git
cd weather-cli
```
2. Create a .env file in the root directory of the project and add your OpenWeatherMap API key:
```
API_KEY=your_api_key_here
```

3. 3.Build the project using Cargo:
```
cargo build --release
```

4. (Optional) Add the executable to your system's PATH to use it from anywhere.

## Usage
To use Weather CLI, run the following command in your terminal:
```
cargo run -- <CITY_NAME>
```
Replace `<CITY_NAME>` with the name of the city you want to fetch weather information for.

## Example:
```
cargo run -- Islamabad
```
The output looks like this:
```
+-----------+-------------+------------+------------+---------------------+---------------------+----------+
| City      | Temperature | Feels Like | Wind Speed | Minimum Temperature | Maximum Temperature | Humidity |
+-----------+-------------+------------+------------+---------------------+---------------------+----------+
| Islamabad | 28.24       | 27.37      | 5.57       | 28.24               | 28.24               | 32       |
+-----------+-------------+------------+------------+---------------------+---------------------+----------+
```