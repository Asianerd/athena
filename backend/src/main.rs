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

        .mount("/project/member/fetch", routes![project::fetch_project_members])
        .mount("/project/member/add", routes![project_members::add])
        .mount("/project/member/remove", routes![project_members::remove])

        .mount("/project/create", routes![project::create])
        .mount("/project/delete", routes![project::delete])
        .mount("/project/edit", routes![project::edit])
        
        .mount("/task/fetch", routes![task::fetch_tasks])
        .mount("/task/create", routes![task::create])
        .mount("/task/delete", routes![task::delete])
        .mount("/task/edit", routes![task::edit])

        .mount("/user/ensure_existance", routes![user::ensure_existance])
}
