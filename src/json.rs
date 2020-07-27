use chrono::{DateTime, Utc};
use md5;
use reqwest;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

const BASE_LINK: &str = "http://api.smitegame.com/smiteapi.svc";

fn read_file_to_string(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    Ok(file_contents)
}

/// Use this to read the dev id and auth key from the token.json file.
fn read_secret(secret_key: &str) -> String {
    let token_file = read_file_to_string("resources/token.json").unwrap();
    let json: Value = serde_json::from_str(&token_file).unwrap();

    json[secret_key].as_str().unwrap().to_string()
}

/// This returns time in YYYYMMDDHHSS format.
/// Required for API queries.
fn get_formatted_time() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y%m%d%H%M%S").to_string()
}

/// Generates an MD5-Hashed signature required for API queries.
fn make_signature(devid: &str, methodname: &str, token: &str, time: &str) -> String {
    let unhashed_signature = format!("{}{}{}{}", devid, methodname, token, time);
    let bytes = unhashed_signature.as_bytes();
    format!("{:x}", md5::compute(bytes))
}

/// Combines keys and signatures with the desired method call to generate
/// a base API query.
/// Most activities require additional text, this is mostly a base and for creating
/// the session.
fn create_link(method: &str) -> String {
    let devid = read_secret("devid");
    let token = read_secret("token");
    let time = get_formatted_time();
    let signature = make_signature(&devid, method, &token, &time);

    format!(
        "{}/{}json/{}/{}/{}",
        BASE_LINK, method, devid, signature, time
    )
}

pub fn fetch_json(link: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::get(link)?.text()?;
    Ok(response)
}
