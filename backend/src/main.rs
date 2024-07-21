#[macro_use] extern crate rocket;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

mod utils;
mod cors;
mod soterius;

mod login_info;
mod user;

mod task;
mod project;
mod project_members;

#[launch]
async fn rocket() -> _ {
    rocket::custom(rocket::config::Config::figment().merge(("port", 8004)))
        .manage(SqlitePool::connect_with(SqliteConnectOptions::new()
            .filename("db")
        ).await.unwrap())
        .attach(cors::CORS)

        .mount("/project/fetch/all", routes![project::fetch_all])
        .mount("/project/fetch/owned", routes![project::fetch_own_projects])
        .mount("/project/fetch/members", routes![project::fetch_project_members])
        .mount("/project/fetch/tasks", routes![project::fetch_tasks])

        .mount("/user/test", routes![user::test])
}
