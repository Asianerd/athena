use std::sync::Mutex;

#[macro_use] extern crate rocket;

mod utils;
mod cors;

mod soterius;

mod database;
mod login_info;
mod user;
mod team;

mod project;
mod group;
mod task;


#[get("/")]
pub fn index() -> String {
    "can you understand me?".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::custom(rocket::config::Config::figment().merge(("port", 8002)))
        .manage(Mutex::new(database::Database::load()))
        .mount("/", routes![index])
        .mount("/save", routes![database::save])
        .mount("/load", routes![database::load])
        .mount("/debug", routes![database::debug])

        .mount("/project/fetch_project", routes![project::fetch_project])
        .mount("/project/create_project", routes![project::create_project])
        .mount("/project/delete_project", routes![project::delete_project])
        .mount("/project/edit_project", routes![project::edit_project])

        .mount("/group/create_group", routes![group::create_group])
        .mount("/group/delete_group", routes![group::delete_group])
        .mount("/group/edit_group", routes![group::edit_group])

        .mount("/task/create_task", routes![task::create_task])
        .mount("/task/delete_task", routes![task::delete_task])
        .mount("/task/edit_task", routes![task::edit_task])
        .mount("/task/assign_task", routes![task::assign_task])
        .mount("/task/toggle_assign_task", routes![task::toggle_assign_task])
        .mount("/task/complete_task", routes![task::complete_task])
        .mount("/task/toggle_complete_task", routes![task::toggle_complete_task])

        .attach(cors::CORS)
}