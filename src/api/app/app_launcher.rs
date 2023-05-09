use rocket::{Build, Rocket, routes};
use crate::api::app::cors::CORS;

use crate::api::authz_card::routes::authz_card_read_router::{hello, authz_cards_all, authz_cards_by_resource};
use crate::api::authz_card::routes::authz_card_write_router::{create, delete};
use crate::api::authz_card::services::authz_card_repository_mongo::AuthzCardRepositoryMongo;
use crate::models::authz_card::errors::custom::CustomError;

pub struct AppLauncher;

impl AppLauncher {
    pub async fn launch_rocket() -> Result<Rocket<Build>, CustomError> {

        AuthzCardRepositoryMongo::new().await
            .map(|taks_mongo_repository| {
                rocket::build()
                    .manage(taks_mongo_repository)
                    .attach(CORS)
                    .mount(
                        "/",
                        routes![
                            hello,
                            authz_cards_all,
                            authz_cards_by_resource,
                            create,
                            delete
                        ]
                    )
            })
            .map_err(|err| CustomError::new(err.to_string().as_str()))
    }
}