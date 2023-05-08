use mongodb::{options::ClientOptions, Client, Collection, bson::Document};
use mongodb::error::Error;

#[async_trait]
pub trait ClientMongoComponent {

    async fn connection() -> Result<Client, Error> {
        let uri: String = std::env::var("MONGO_URI")
            .expect("mongodb://localhost:27017");

        let mut client_option = ClientOptions::parse(uri).await?;
        client_option.app_name = Some(String::from("Authz"));
        Client::with_options(client_option)
    }

    async fn collection_authz_card() -> Result<Collection<Document>, mongodb::error::Error> {
        Ok(
            Self::connection()
                .await?
                .database("authz")
                .collection("authz_card")
        )
    }
}
