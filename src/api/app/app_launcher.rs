use rocket::{Build, Rocket, routes};
use crate::api::app::cors::CORS;

use crate::api::tasks::routes::task_read_router::{hello, tasks};
use crate::api::tasks::routes::task_write_router::{create, running_task, pending_all_task};
use crate::api::tasks::services::tasks_repository_mongo::TasksRepositoryMongo;
use crate::models::tasks::errors::custom::CustomError;

pub struct AppLauncher;

impl AppLauncher {
    pub async fn launch_rocket() -> Result<Rocket<Build>, CustomError> {

        TasksRepositoryMongo::new().await
            .map(|taks_mongo_repository| {
                rocket::build()
                    .manage(taks_mongo_repository)
                    .attach(CORS)
                    .mount(
                        "/",
                        routes![
                            hello,
                            tasks,
                            create,
                            running_task,
                            pending_all_task
                        ]
                    )
            })
            .map_err(|err| CustomError::new(err.to_string().as_str()))
    }
}