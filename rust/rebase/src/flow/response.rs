use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// TODO: Restore when we break backwards compat
// #[derive(Deserialize, JsonSchema, Serialize)]
// pub struct BasicResponse {
//     pub statement: String,
// }

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct PostResponse {
    pub statement: String,
    pub delimitor: String,
}
