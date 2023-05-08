use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateAuthzCardCommand {
    pub description: String,
    pub resource: String,
    pub action: String,
    pub users: Vec<String>
}
