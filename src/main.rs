#[allow(unused_imports)]
#[macro_use] extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

use std::io::Read;
use serde_json::Value;
use url::Url;

#[allow(unused_must_use)]
fn get_json(zip: &str) -> Value {
    let key = "1aa84f76ad87af18";
    let url_str = format!("http://api.wunderground.com/api/{}/conditions/q/{}.json", key, zip);
    let url = Url::parse(url_str.as_str()).unwrap();
    let mut resp = reqwest::get(Url::parse(url.as_str()).unwrap()).unwrap();
    assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content);

    serde_json::from_str(content.as_str()).unwrap()
}

fn main() {
    let json = get_json("97068");
    println!("{:?}", json);
}
