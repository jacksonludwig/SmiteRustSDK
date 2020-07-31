use super::json;

/// Create an object to make queries with its given id.
pub struct QueryBuilder {
    session_id: String,
}

impl QueryBuilder {
    /// Use the session id and current time to make a new builder.
    pub fn new(session_id: String) -> QueryBuilder {
        QueryBuilder { session_id }
    }

    /// Make a "getplayer" query to the API.
    pub fn get_player(&self, name: &str) -> Result<String, reqwest::Error> {
        let link = format!(
            "{}/{}",
            json::create_link("getplayer", &self.session_id, &json::get_formatted_time()),
            name
        );
        json::fetch_json(&link)
    }

    /// Make a "getgods" query to the API.
    pub fn get_gods(&self) -> Result<String, reqwest::Error> {
        let link = format!(
            "{}/1",
            json::create_link("getgods", &self.session_id, &json::get_formatted_time())
        );
        json::fetch_json(&link)
    }
}
