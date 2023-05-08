use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct JsonDataResponse {
    pub message: String
}

impl JsonDataResponse {
    pub fn new(message: &str) -> Self {
        JsonDataResponse {
            message: message.into()
        }
    }
}
