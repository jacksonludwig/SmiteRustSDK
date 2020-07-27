use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Session {
    ret_msg: String,
    session_id: String,
    timestamp: String,
}

impl Session {
    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
}
