use serde_json;

fn get_weather_data(key: &str) -> Result<(), ureq::Error> {
    let url = "https://weatherapi.com/v1/current.json?key=$$KEY$$&q=auto:ip&aqi=yes";
    let json: serde_json::Value = ureq::get(url.replace("$$KEY$$", key).as_str())
        .call()?
        .into_json()?;
    Ok(())
}

fn main(){
    //println!("{}", weatherapi_json["current"]["temp_f"]);
}