use mongodb::bson::{doc, Document};
use mongodb::Collection;
use mongodb::error::Error;
use mongodb::results::InsertOneResult;
use rocket::futures::{TryFutureExt, TryStreamExt};

use crate::api::app::mongo_component::ClientMongoComponent;
use crate::api::authz_card::entities::authz_card_dbo::AuthzCardDbo;
use crate::core::authz_card::entities::authz_card::AuthzCard;
use crate::core::authz_card::services::authz_card_repository::AuthzCardRepository;
use crate::models::authz_card::errors::custom::CustomError;

pub struct AuthzCardRepositoryMongo {
    pub collection: Collection<Document>,
}

impl ClientMongoComponent for AuthzCardRepositoryMongo {}

#[async_trait]
impl AuthzCardRepository for AuthzCardRepositoryMongo {
    async fn insert_authz_card(&self, authz_card: AuthzCard) -> Result<(), CustomError> {
        if !self.exist(&authz_card).await {
            self
                .insert_task_without_check(&authz_card)
                .await
                .map(|_| ())
        } else {
            self.delete_authz_card(authz_card.resource.as_str(), authz_card.action.as_str())
                .and_then(|_| self.insert_task_without_check(&authz_card))
                .await
                .map(|_| ())
        }
    }

    async fn delete_authz_card(&self, resource: &str, action: &str) -> Result<(), CustomError> {
        if self.fetch_one_by_id(resource, action).await.is_ok() {
            self.delete_task_without_check(resource, action).await
        } else {
            Err(CustomError::new("la tache n'éxiste pas"))
        }
    }

    async fn fetch_many(&self) -> Vec<AuthzCard> {
        match self.find_all().await {
            Ok(tasks) => tasks,
            _ => {
                eprintln!("une erreur est survenue lors de la lecture");
                vec![]
            }
        }
    }

    async fn fetch_many_by_id(&self, resource: &str) -> Vec<AuthzCard> {
        self.fetch_many()
            .await
            .into_iter()
            .filter(|card| card.resource.as_str() == resource)
            .collect::<Vec<_>>()
    }

    async fn fetch_one_by_id(&self, resource: &str, action: &str) -> Result<AuthzCard, CustomError> {
        self.collection
            .find_one(
                Some(
                    doc! {
                        "resource": resource,
                        "action": action
                    }
                ),
                None
            )
            .await
            .map(|doc_opt| {
                doc_opt
                    .map(|doc| {
                        let authz_card_dbo: AuthzCardDbo = doc.into();
                        let authz_card: AuthzCard = authz_card_dbo.into();
                        Ok(authz_card)
                    })
                    .unwrap_or(Err(CustomError::new("impossible de recupere la card")))
            })
            .unwrap_or_else(|err| Err(CustomError::new(format!("{}", err.to_string()).as_str())))
    }


    // async fn change_state(&self, id: &str, state: String) -> Result<(), CustomError> {
    //     let filter = doc! {
    //         "id": id
    //     };
    //     let update = doc! {
    //         "$set": {
    //             "state": state.as_str()
    //         }
    //     };
    //     self.collection
    //         .update_one(filter, update, None)
    //         .await
    //         .map(|_| ())
    //         .map_err(|_| CustomError::new("erreur lors de l'update"))
    // }
}

impl AuthzCardRepositoryMongo {

    async fn find_all(&self) -> Result<Vec<AuthzCard>, Error> {
        Ok(
            self.collection
                .find(None, None)
                .await?
                .try_collect::<Vec<Document>>()
                .await?
                .into_iter()
                .map(|doc| doc.into())
                .map(|dbo: AuthzCardDbo| dbo.into())
                .collect::<Vec<AuthzCard>>()
                .into()
        )
    }

    pub async fn new() -> Result<Self, mongodb::error::Error> {
        Ok(
            Self {
                collection: {
                    Self::collection_authz_card().await?
                }
            }
        )
    }

    async fn exist(&self, task: &AuthzCard) -> bool {
        self.fetch_one_by_id(task.resource.as_str(), task.action.as_str())
            .await
            .is_ok()
    }

    async fn insert_task_without_check(&self, task: &AuthzCard) -> Result<InsertOneResult, CustomError> {
        let document: Document = task.clone().into();
        self.collection
            .insert_one(document, None)
            .map_err(|_| CustomError::new("erreur lors de l'insertion en base"))
            .await
    }

    async fn delete_task_without_check(&self, resource: &str, action: &str) -> Result<(), CustomError> {
        let document: Document = doc! {
            "resource": resource,
            "action": action
        };
        self.collection
            .delete_one(document, None)
            .await
            .map(|_| ())
            .map_err(|_| CustomError::new("erreur lors de la suppression"))
    }

}
