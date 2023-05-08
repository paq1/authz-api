#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};
use crate::api::app::app_launcher::AppLauncher;

mod api;
mod core;
mod models;

#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv::dotenv().ok();
    AppLauncher::launch_rocket()
        .await
        .unwrap()
}
