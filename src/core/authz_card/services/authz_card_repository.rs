use crate::core::authz_card::entities::authz_card::AuthzCard;
use crate::models::authz_card::errors::custom::CustomError;

#[async_trait]
pub trait AuthzCardRepository {
    async fn insert_authz_card(&self, authz_card: AuthzCard) -> Result<(), CustomError>;
    async fn delete_authz_card(&self, resource: &str, action: &str) -> Result<(), CustomError>;
    async fn fetch_many(&self) -> Vec<AuthzCard>;
    async fn fetch_many_by_id(&self, resource: &str) -> Vec<AuthzCard>;
    async fn fetch_one_by_id(&self, resource: &str, action: &str) -> Result<AuthzCard, CustomError>;
}