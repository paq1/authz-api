use mongodb::bson::{Bson, Document};
use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};
use crate::core::authz_card::entities::authz_card::AuthzCard;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthzCardDbo {
    pub _id: ObjectId,
    pub description: String,
    pub resource: String,
    pub action: String,
    pub users: Vec<String>
}

impl From<AuthzCardDbo> for AuthzCard {
    fn from(value: AuthzCardDbo) -> Self {
        Self {
            description: value.description,
            resource: value.resource,
            action: value.action,
            users: value.users
        }
    }
}

impl From<Document> for AuthzCardDbo {
    fn from(value: Document) -> Self {
        mongodb::bson::from_bson(Bson::Document(value))
            .unwrap()
    }
}