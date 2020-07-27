mod json;
mod response;

fn main() {
    let session_id = json::make_session().unwrap();
    let player_link = format!(
        "{}/{}",
        json::create_link("getplayer", &session_id, &json::get_formatted_time()),
        "SwiggedySwoody"
    );
    println!("{}", player_link);
}
