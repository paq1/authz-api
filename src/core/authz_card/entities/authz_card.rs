use mongodb::bson::{doc, Document};
use crate::models::authz_card::views::authz_card_view::AuthzCardView;

#[derive(Clone, Debug)]
pub struct AuthzCard {
    pub description: String,
    pub resource: String,
    pub action: String,
    pub users: Vec<String>
}


impl From<AuthzCard> for AuthzCardView {
    fn from(value: AuthzCard) -> Self {
        Self {
            description: value.description,
            resource: value.resource,
            action: value.action,
            users: value.users
        }
    }
}

impl From<AuthzCard> for Document {
    fn from(value: AuthzCard) -> Self {
        doc! {
            "description": value.description,
            "resource": value.resource,
            "action": value.action,
            "users": value.users
        }
    }
}