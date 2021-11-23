use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    message_id: String,
    channel_id: String,
}