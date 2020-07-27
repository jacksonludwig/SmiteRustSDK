use chrono::{DateTime, Utc};
use md5;
use reqwest;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

use super::response::Session;

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
pub fn get_formatted_time() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y%m%d%H%M%S").to_string()
}

/// Generates an MD5-Hashed signature required for API queries.
fn make_signature(devid: &str, methodname: &str, token: &str, time: &str) -> String {
    let unhashed_signature = format!("{}{}{}{}", devid, methodname, token, time);
    let bytes = unhashed_signature.as_bytes();
    format!("{:x}", md5::compute(bytes))
}

/// Use signature to create the session link.
/// Returns the link and the signature.
pub fn create_session_link() -> String {
    let devid = read_secret("devid");
    let token = read_secret("token");
    let method = "createsession";
    let time = get_formatted_time();
    let signature = make_signature(&devid, method, &token, &time);

    format!(
        "{}/{}json/{}/{}/{}",
        BASE_LINK, method, devid, signature, time
    )
}

/// Create the session and return the session id.
pub fn make_session() -> Result<String, reqwest::Error> {
    let link = create_session_link();
    let response = fetch_json(&link)?;
    let session: Session = serde_json::from_str(&response).unwrap();
    Ok(session.get_session_id().to_string())
}

/// Use session id to create links to any method call.
pub fn create_link(method: &str, session_id: &str, timestamp: &str) -> String {
    let devid = read_secret("devid");
    let time = get_formatted_time();
    let token = read_secret("token");
    let signature = make_signature(&devid, method, &token, &time);

    format!(
        "{}/{}json/{}/{}/{}/{}",
        BASE_LINK, method, devid, signature, session_id, timestamp
    )
}

pub fn fetch_json(link: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::get(link)?.text()?;
    Ok(response)
}
