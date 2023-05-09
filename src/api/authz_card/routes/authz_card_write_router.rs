use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::authz_card::services::authz_card_repository_mongo::AuthzCardRepositoryMongo;
use crate::core::authz_card::entities::authz_card::AuthzCard;
use crate::core::authz_card::services::authz_card_repository::AuthzCardRepository;
use crate::models::authz_card::commands::create_authz_card_command::CreateAuthzCardCommand;
use crate::models::authz_card::views::json_data_response::JsonDataResponse;

#[post("/authz_card/commands/create", data="<create_command>")]
pub async fn create(
    authz_card_repository: &State<AuthzCardRepositoryMongo>,
    create_command: Json<CreateAuthzCardCommand>
) -> Result<Json<JsonDataResponse>, status::Custom<Json<JsonDataResponse>>> {
    let cmd = create_command.0;
    authz_card_repository
        .insert_authz_card(
            AuthzCard {
                description: cmd.description,
                resource: cmd.resource,
                action: cmd.action,
                users: cmd.users
            }
        )
        .await
        .map(|_| Json(JsonDataResponse::new("inserted")))
        .map_err(|err| {
            status::Custom(
                Status::BadRequest,
                Json(
                    JsonDataResponse::new(err.message.as_str())
                )
            )
        })
}

#[delete("/authz_card/commands/delete/<resource>/<action>")]
pub async fn delete(
    authz_card_repository: &State<AuthzCardRepositoryMongo>,
    resource: &str,
    action: &str
) -> Result<Json<JsonDataResponse>, status::Custom<Json<JsonDataResponse>>> {

    authz_card_repository
        .delete_authz_card(
            resource, action
        )
        .await
        .map(|_| Json(JsonDataResponse::new("deleted")))
        .map_err(|err| {
            status::Custom(
                Status::BadRequest,
                Json(
                    JsonDataResponse::new(err.message.as_str())
                )
            )
        })
}
