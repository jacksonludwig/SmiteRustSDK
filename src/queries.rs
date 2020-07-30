use super::json;

/// Create an object to make queries with its given id and timestamp.
pub struct QueryBuilder {
    session_id: String,
    timestamp: String,
}

impl QueryBuilder {
    /// Use the session id and timestamp to make a new builder.
    pub fn new(session_id: String, timestamp: String) -> QueryBuilder {
        QueryBuilder {
            session_id,
            timestamp,
        }
    }

    /// Make a "getplayer" query to the API.
    pub fn get_player(&self, name: &str) -> Result<String, reqwest::Error> {
        let link = format!(
            "{}/{}",
            json::create_link("getplayer", &self.session_id, &self.timestamp),
            name
        );
        json::fetch_json(&link)
    }
}
