#[allow(unused_imports)]
#[macro_use] extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

use std::io;
use std::io::Read;
use serde_json::Value;
use url::Url;

#[allow(unused_must_use)]
fn get_json(zip: &str) -> Value {
    let key = "1aa84f76ad87af18";
    let url_str = format!("http://api.wunderground.com/api/{}/conditions/q/{}.json", key, zip);
    let url = Url::parse(url_str.as_str()).unwrap();
    let mut resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content);

    serde_json::from_str(content.as_str()).unwrap()
}

fn main() {
    println!("What area would you like a forecast for? ");
    let mut zip = String::new();

    io::stdin()
        .read_line(&mut zip)
        .expect("Failed to read line");
    let zip = zip.trim();

    let json = get_json(zip);
    let chars_to_trim: &[char] = &['"'];
    let ref feels_like = json["current_observation"]["feelslike_string"].to_string();
    let feels_like = feels_like.trim_matches(chars_to_trim);
    let ref weather = json["current_observation"]["weather"].to_string();
    let weather = weather.trim_matches(chars_to_trim);
    println!("Weather for {} is currently {}, and temp feels like {}", zip, weather, feels_like);
}
