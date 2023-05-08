use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use crate::api::authz_card::services::authz_card_repository_mongo::AuthzCardRepositoryMongo;
use crate::core::authz_card::services::authz_card_repository::AuthzCardRepository;

use crate::models::authz_card::views::json_data_response::JsonDataResponse;
use crate::models::authz_card::views::authz_card_view::AuthzCardView;

#[get("/hello-world")]
pub async fn hello() -> Result<Json<JsonDataResponse>, status::Custom<Json<JsonDataResponse>>> {
    Ok(Json(JsonDataResponse::new("hello world")))
}

#[get("/authz_cards")]
pub async fn authz_cards_all(
    authz_card_repository: &State<AuthzCardRepositoryMongo>
) -> Result<Json<Vec<AuthzCardView>>, status::Custom<Json<JsonDataResponse>>> {
    Ok(
        Json(
            authz_card_repository
                .fetch_many()
                .await
                .into_iter()
                .map(|task| {
                    task.into()
                })
                .collect::<Vec<_>>()
        )
    )
}

#[get("/authz_cards_by_resource/<resource>")]
pub async fn authz_cards_by_resource(
    authz_card_repository: &State<AuthzCardRepositoryMongo>,
    resource: &str
) -> Result<Json<Vec<AuthzCardView>>, status::Custom<Json<JsonDataResponse>>> {
    Ok(
        Json(
            authz_card_repository
                .fetch_many_by_id(resource)
                .await
                .into_iter()
                .map(|task| {
                    task.into()
                })
                .collect::<Vec<_>>()
        )
    )
}