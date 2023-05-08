use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthzCardView {
    pub description: String,
    pub resource: String,
    pub action: String,
    pub users: Vec<String>
}
