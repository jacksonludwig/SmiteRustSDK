mod json;
mod response;

// fn main() {
//     let session = json::make_session().unwrap();
//     println!("Session started with id: {}", &session.session_id);
//     println!("Session started with timestamp: {}", &session.timestamp);

//     let link = format!(
//         "{}/{}",
//         json::create_link(
//             "getplayer",
//             &session.session_id,
//             &json::get_formatted_time()
//         ),
//         "Cherryo",
//     );
//     println!("{}", link);
// }
