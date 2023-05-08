use mongodb::{options::ClientOptions, Client, Collection, bson::Document};
use mongodb::error::Error;

#[async_trait]
pub trait ClientMongoComponent {

    async fn connection() -> Result<Client, Error> {
        let uri: String = std::env::var("MONGO_URI")
            .expect("mongodb://localhost:27017");

        let mut client_option = ClientOptions::parse(uri).await?;
        client_option.app_name = Some(String::from("Scheduler"));
        Client::with_options(client_option)
    }

    async fn collection_tasks() -> Result<Collection<Document>, mongodb::error::Error> {
        Ok(
            Self::connection()
                .await?
                .database("scheduler")
                .collection("tasks")
        )
    }
}
