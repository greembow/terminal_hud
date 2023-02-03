use std::fs;
use serde::Deserialize;
use std::process::exit;

#[derive(Deserialize)]
struct Config {
    key: String,
    unit: String,
}

fn get_config() -> Config {
    let file_path = "/home/greembow/.config/terminal_hud/config.toml";
    let file_contents = match fs::read_to_string(file_path) {
        Err(e) => {
            println!("Unable to open config, error follows. {:?}", e);
            exit(1);
        },
        Ok(x) => x,
    };
    let config: Config = match toml::from_str(file_contents.as_str()) {
        Err(e) => {
            println!("Unable to process config, error follows. {:?}", e);
            exit(2)
        },
        Ok(x) => x,
    };
    return config;
}

fn get_weather_data(key: String) -> serde_json::Value {
    let url = "https://api.weatherapi.com/v1/current.json?key=$$KEY$$&q=auto:ip&aqi=yes";
    let result: ureq::Response = match ureq::get(url.replace("$$KEY$$", key.as_str()).as_str()).call() {
        Err(e) => {
            println!("Unable to get weather data, error follows. {:?}", e);
            exit(3);
        },
        Ok(x) => x,
    };
    match result.into_json() {
        Err(e) => {
            println!("Unable to convert weather data into JSON, error follows. {:?}", e);
            exit(4);
        }
        Ok(x) => return x,
    };
}

fn main() {
    let config = get_config();
    let weather_data = get_weather_data(config.key);

    if config.unit.to_lowercase() == "c" {
        println!("Temperature = {}", weather_data["current"]["temp_c"])
    } else if config.unit.to_lowercase() == "f" {
        println!("Temperature = {}", weather_data["current"]["temp_f"])
    } else {
        println!("Invalid temperature unit, options are `c` for celsius, and `f` for farenheight.")
    }
}