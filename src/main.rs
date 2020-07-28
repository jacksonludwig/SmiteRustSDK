mod json;
mod response;

fn main() {
    let session = json::make_session().unwrap();
    println!("Session started with id: {}", &session.session_id);

    let player_link = format!(
        "{}/{}",
        json::create_link(
            "getplayer",
            &session.session_id,
            &json::get_formatted_time()
        ),
        "SwiggedySwoody"
    );
    println!("{}", player_link);
}
