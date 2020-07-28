use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub ret_msg: String,
    pub session_id: String,
    pub timestamp: String,
}
